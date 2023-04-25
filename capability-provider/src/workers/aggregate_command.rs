use async_nats::jetstream::Context;
use tracing::{debug, error, instrument, trace};

use crate::{
    config::InterestDeclaration,
    consumers::WorkError,
    events::publish_es_event,
    eventsourcing::{AggregateService, AggregateServiceSender, StatefulCommand},
    natsclient::AckableMessage,
    state::EntityState,
};

use crate::consumers::{RawCommand, WorkResult, Worker};

// TODO: add an AggregateEventWorker

pub struct AggregateCommandWorker {
    pub nc: async_nats::Client,
    pub context: Context,
    pub interest: InterestDeclaration,
    pub state: EntityState,
}

impl AggregateCommandWorker {
    pub fn new(
        nc: async_nats::Client,
        context: Context,
        interest: InterestDeclaration,
        state: EntityState,
    ) -> Self {
        AggregateCommandWorker {
            nc,
            context,
            interest,
            state,
        }
    }
}

#[async_trait::async_trait]
impl Worker for AggregateCommandWorker {
    type Message = RawCommand;

    /// Commands always go to aggregates, and their topic filters are always explicit to one topic, so we
    /// don't need as much ceremony around interest-based dispatch for commands as we do for events
    #[instrument(level = "debug", skip_all, fields(entity_name = self.interest.entity_name))]
    async fn do_work(&self, mut message: AckableMessage<Self::Message>) -> WorkResult<()> {
        debug!(command = ?message.as_ref(), "Handling received command");

        let state = self
            .state
            .fetch_state(
                &self.interest.role,
                &self.interest.entity_name,
                &message.key,
            )
            .await
            .map_err(|e| WorkError::NatsError(format!("Failed to load state: {e}").into()))?;
        if let Some(ref vec) = state {
            trace!("Loaded pre-existing state - {} bytes", vec.len());
        }

        // NOTE: you can't invoke `for_actor` without an active provider_main. This will even implicitly attempt
        // to create one, so be wary of using this worker for tests.
        let target = AggregateServiceSender::for_actor(&self.interest.link_definition);
        let cmd = StatefulCommand {
            aggregate: self.interest.entity_name.to_string(),
            command_type: message.command_type.to_string(),
            key: message.key.to_string(),
            state,
            payload: serde_json::to_vec(&message.data).map_err(|e| {
                WorkError::Other(format!(
                    "Raw inbound command payload could not be converted to vec: {e}"
                ))
            })?,
        };
        let ctx = wasmbus_rpc::provider::prelude::Context::default();
        trace!(
            "Dispatching command {} to {}",
            cmd.command_type,
            self.interest.actor_id
        );
        let outbound_events = target.handle_command(&ctx, &cmd).await.map_err(|e| {
            WorkError::Other(format!(
                "Aggregate {} ({}) failed to handle command, type '{}', key '{}', ({} bytes): {:?}",
                self.interest.actor_id,
                cmd.aggregate,
                cmd.command_type,
                cmd.key,
                cmd.payload.len(),
                e
            ))
        })?;
        trace!("Command handler produced {} events", outbound_events.len());

        let cmd_type = cmd.command_type.clone();

        // Reminder that aggregates don't modify their own state when processing commands. That can only
        // happen when handling events.

        // TODO: check for lease expiration (skip outbound pub if callee timeout would have already expired) - thanks Victor

        for evt in outbound_events {
            let evt_type = evt.event_type.clone();
            if let Err(_e) = publish_es_event(&self.nc, evt)
                .await
                .map_err(|e| WorkError::NatsError(e.into()))
            {
                error!(
                    "Failed to publish outbound event {evt_type} in response to command {cmd_type}"
                );
                message.nack().await;
                return Ok(());
            }
        }

        // Now that the outbound has been processed, ack the inbound (which deletes the command from the work queue CC_COMMANDS stream)
        message.ack().await.map_err(|e| WorkError::NatsError(e))?;

        Ok(())
    }
}
