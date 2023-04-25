use async_nats::jetstream::Context;
use cloudevents::Event as CloudEvent;
use tracing::{debug, error, instrument, trace, warn};
use wasmbus_rpc::error::RpcError;

use crate::{
    config::InterestDeclaration,
    consumers::WorkError,
    eventsourcing::{
        AggregateService, AggregateServiceSender, Event as ConcordanceEvent, EventWithState,
        StateAck,
    },
    natsclient::AckableMessage,
    state::EntityState,
};

use crate::consumers::{WorkResult, Worker};

pub struct AggregateEventWorker {
    pub nc: async_nats::Client,
    pub context: Context,
    pub interest: InterestDeclaration,
    pub state: EntityState,
}

impl AggregateEventWorker {
    pub fn new(
        nc: async_nats::Client,
        context: Context,
        interest: InterestDeclaration,
        state: EntityState,
    ) -> Self {
        AggregateEventWorker {
            nc,
            context,
            interest,
            state,
        }
    }
}

#[async_trait::async_trait]
impl Worker for AggregateEventWorker {
    type Message = CloudEvent;

    #[instrument(level = "debug", skip_all, fields(actor_id = self.interest.actor_id))]
    async fn do_work(&self, mut message: AckableMessage<Self::Message>) -> WorkResult<()> {
        debug!(event = ?message.as_ref(), "Aggregate handling received event");

        let self_id = &self.interest.actor_id;
        let ce: ConcordanceEvent = message.inner.clone().into();
        if !self.interest.is_interested_in_event(&ce) {
            trace!(
                "Aggregate is not interested in event '{}' on stream '{}'. Acking and moving on.",
                ce.event_type,
                ce.stream
            );
            message.ack().await.map_err(|e| WorkError::NatsError(e))?;
            return Ok(());
        }
        let evt_payload: serde_json::Value =
            serde_json::from_slice(&ce.payload).unwrap_or_default();
        let key = self.interest.extract_key_value_from_payload(&evt_payload);
        let state = if !key.is_empty() {
            self.state
                .fetch_state(&self.interest.role, &self.interest.entity_name, &key)
                .await
                .map_err(|e| {
                    WorkError::NatsError(
                        format!("Failed to load state for aggregate {self_id} : {e}").into(),
                    )
                })?
        } else {
            warn!("Key field {} not found on incoming event. This indicates either bad data or potentially side-effectful behavior",
                &self.interest.key_field);
            None
        };

        let ctx = wasmbus_rpc::provider::prelude::Context::default();
        let target = AggregateServiceSender::for_actor(&self.interest.link_definition);
        let ews = EventWithState {
            event: ce.clone(),
            state: state.clone(),
        };
        trace!(
            "About to apply event {} to target {}",
            ce.event_type,
            self.interest.actor_id
        );

        let state_ack = target.apply_event(&ctx, &ews).await;
        // Failures will result in a message nack
        self.adjust_state(&mut message, state_ack, &key).await?;

        Ok(())
    }
}

impl AggregateEventWorker {
    async fn adjust_state(
        &self,
        msg: &mut AckableMessage<CloudEvent>,
        state_ack: Result<StateAck, RpcError>,
        key: &str,
    ) -> WorkResult<()> {
        match state_ack {
            Ok(StateAck {
                succeeded: true,
                state: Some(s),
                ..
            }) => self.save_state(msg, key, s).await?,
            Ok(StateAck {
                succeeded: true,
                state: None,
                ..
            }) => self.remove_state(msg, key).await?,
            Ok(StateAck {
                succeeded: false,
                error: Some(e),
                ..
            }) => {
                error!("Failed to apply event to target actor: {e}");
                msg.nack().await;
            }
            Ok(StateAck {
                succeeded: false,
                error: None,
                ..
            }) => {
                error!("Failed to apply event to target actor: unspecified error");
                msg.nack().await;
            }
            Err(e) => {
                error!("Failed to apply event to target actor: {e}");
                msg.nack().await;
            }
        }
        Ok(())
    }

    async fn save_state(
        &self,
        msg: &mut AckableMessage<CloudEvent>,
        key: &str,
        data: Vec<u8>,
    ) -> WorkResult<()> {
        if key.is_empty() {
            return Ok(());
        }

        let self_id = &self.interest.actor_id;
        match self
            .state
            .write_state(&self.interest.role, &self.interest.entity_name, key, data)
            .await
        {
            Ok(_) => {
                trace!("Aggregate {self_id} state written. Acknowledging event.");
                msg.ack().await.map_err(|e| WorkError::NatsError(e))?;
            }
            Err(e) => {
                error!(
                    "Failed to write state after event application, aggregate is now outdated: {e}"
                );
                msg.nack().await;
            }
        }

        Ok(())
    }

    async fn remove_state(
        &self,
        msg: &mut AckableMessage<CloudEvent>,
        key: &str,
    ) -> WorkResult<()> {
        if key.is_empty() {
            return Ok(());
        }
        let self_id = &self.interest.actor_id;
        match self
            .state
            .remove_state(&self.interest.role, &self.interest.entity_name, key)
            .await
        {
            Ok(_) => {
                trace!("Aggregate {self_id} state deleted.");
                msg.ack().await.map_err(|e| WorkError::NatsError(e))?;
            }
            Err(e) => {
                error!("Failed to delete aggregate {self_id} state: {e}");
                msg.nack().await;
            }
        }
        Ok(())
    }
}
