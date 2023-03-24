use async_nats::jetstream::Context;
use cloudevents::Event as CloudEvent;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::{
    config::{ActorInterest, InterestDeclaration, ProcessManagerLifetime},
    consumers::{RawCommand, WorkError},
    events::publish_raw_command,
    eventsourcing::{
        Event as ConcordanceEvent, EventWithState, ProcessManagerAck, ProcessManagerService,
        ProcessManagerServiceSender, StateAck, StatefulCommand,
    },
    natsclient::AckableMessage,
    state::EntityState,
};

use crate::consumers::{WorkResult, Worker};

pub struct ProcessManagerWorker {
    pub nc: async_nats::Client,
    pub context: Context,
    pub interest: InterestDeclaration,
    pub state: EntityState,
}

impl ProcessManagerWorker {
    pub fn new(
        nc: async_nats::Client,
        context: Context,
        interest: InterestDeclaration,
        state: EntityState,
    ) -> ProcessManagerWorker {
        ProcessManagerWorker {
            nc,
            context,
            interest,
            state,
        }
    }
}

#[async_trait::async_trait]
impl Worker for ProcessManagerWorker {
    type Message = CloudEvent;

    /// Ingest a cloud event, dispatch to the target actor, and dispatch the returned list
    /// of commands. State is passed in with the event and state is persisted/removed based
    /// on the ack from the process manager
    async fn do_work(&self, mut message: AckableMessage<Self::Message>) -> WorkResult<()> {
        debug!(event = ?message.as_ref(), "Process manager handling received event");

        let self_id = &self.interest.actor_id;
        let ce: ConcordanceEvent = message.inner.clone().into();
        if !self.interest.is_interested_in_event(&ce) {
            // at the moment we can't declare per-event subscriptions in NATS, so PM consumers listen to
            // all events, so we should silently ack them
            message.ack().await.map_err(|e| WorkError::NatsError(e))?;
            return Ok(());
        }

        let evt_payload: serde_json::Value =
            serde_json::from_slice(&ce.payload).unwrap_or_default();
        let key = self.interest.extract_key_value_from_payload(&evt_payload);

        let state = if !key.is_empty() {
            match &self.interest.interest {
                ActorInterest::ProcessManager(pm_life)
                    if pm_life.event_starts_new_process(&ce.event_type) =>
                {
                    // Never deliver state to a process manager on its lifetime-start event
                    None
                }
                _ => self
                    .state
                    .fetch_state(&self.interest.role, &self.interest.entity_name, &key)
                    .await
                    .map_err(|e| {
                        WorkError::NatsError(format!("Failed to load state: {e}").into())
                    })?,
            }
        } else {
            None
        };

        if let Some(ref vec) = state {
            trace!("Loaded pre-existing state - {} bytes", vec.len());
        }

        let target = ProcessManagerServiceSender::for_actor(&self.interest.link_definition);
        let ctx = wasmbus_rpc::provider::prelude::Context::default();
        trace!(
            "Dispatching event '{}' to process manager '{}'",
            ce.event_type,
            self.interest.actor_id
        );
        let inbound_event = EventWithState {
            event: ce.clone(),
            state,
        };
        let pm_ack = target
            .handle_event(&ctx, &inbound_event)
            .await
            .map_err(|e| {
                WorkError::Other(format!(
                    "Process manager {} failed to process event {}: {}",
                    self_id, ce.event_type, e
                ))
            })?;

        // These will nack upon failure and return Err, so the following ack will
        // never get called
        self.dispatch_commands(&mut message, &pm_ack).await?;
        self.save_state(&mut message, &pm_ack, &key).await?;

        message.ack().await.map_err(|e| WorkError::NatsError(e))?;

        Ok(())
    }
}

impl ProcessManagerWorker {
    async fn dispatch_commands(
        &self,
        msg: &mut AckableMessage<CloudEvent>,
        ack: &ProcessManagerAck,
    ) -> WorkResult<()> {
        for cmd in &ack.commands {
            let rawcmd = RawCommand {
                command_type: cmd.command_type.to_string(),
                key: cmd.aggregate_key.to_string(),
                data: serde_json::from_slice(&cmd.json_payload).unwrap_or_default(),
            };
            if let Err(e) = publish_raw_command(&self.nc, rawcmd, &cmd.aggregate_stream).await {
                msg.nack().await;
                return Err(WorkError::NatsError(e.into()));
            }
        }
        Ok(())
    }

    async fn save_state(
        &self,
        msg: &mut AckableMessage<CloudEvent>,
        ack: &ProcessManagerAck,
        key: &str,
    ) -> WorkResult<()> {
        let self_id = self.interest.actor_id.clone();

        if let Some(state) = ack.state.clone() {
            match self
                .state
                .write_state(&self.interest.role, &self.interest.entity_name, &key, state)
                .await
            {
                Ok(_) => {
                    trace!("Process manager {self_id} state written.");
                }
                Err(e) => {
                    error!("Failed to write state after event application, aggregate is now outdated: {e}");
                    msg.nack().await;
                    return Err(WorkError::NatsError(e.into()));
                }
            }
        } else {
            match self
                .state
                .remove_state(&self.interest.role, &self.interest.entity_name, key)
                .await
            {
                Ok(_) => {
                    trace!("Process manager {self_id} state deleted on demand.");
                }
                Err(e) => {
                    error!("Failed to delete process manager {self_id} state: {e}");
                    msg.nack().await;
                    return Err(WorkError::NatsError(e.into()));
                }
            }
        }
        Ok(())
    }
}
