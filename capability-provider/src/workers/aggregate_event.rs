use async_nats::jetstream::Context;
use cloudevents::Event as CloudEvent;
use tracing::{debug, error, info, instrument, trace};

use crate::{
    config::InterestDeclaration,
    consumers::WorkError,
    eventsourcing::{
        AggregateService, AggregateServiceSender, Event as ConcordanceEvent, EventWithState,
        StateAck, StatefulCommand,
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
        debug!(command = ?message.as_ref(), "Aggregate handling received event");

        let self_id = &self.interest.actor_id;
        let ce: ConcordanceEvent = message.inner.clone().into();
        if !self.interest.is_interested_in_event(&ce) {
            debug!("Aggregate is not interested");
            return Ok(());
        }
        let state = self
            .state
            .fetch_state(&self.interest.role, &self.interest.entity_name, &ce.key)
            .await
            .map_err(|e| {
                WorkError::NatsError(
                    format!("Failed to load state for aggregate {self_id} : {e}").into(),
                )
            })?;

        let ctx = wasmbus_rpc::provider::prelude::Context::default();
        let target = AggregateServiceSender::for_actor(&self.interest.link_definition);
        let ews = EventWithState {
            event: ce.clone(),
            state: state.clone(),
        };
        debug!(
            "About to apply event {} to target {}",
            ce.event_type, self.interest.actor_id
        );
        match target.apply_event(&ctx, &ews).await {
            Ok(StateAck {
                succeeded: true,
                state: Some(s),
                ..
            }) => {
                match self
                    .state
                    .write_state(&self.interest.role, &self.interest.entity_name, &ce.key, s)
                    .await
                {
                    Ok(_) => {
                        trace!("Aggregate {self_id} state written. Acknowledging event.");
                        message.ack().await.map_err(|e| WorkError::NatsError(e))?;
                    }
                    Err(e) => {
                        error!("Failed to write state after event application, aggregate is now outdated: {e}");
                        message.nack().await;
                    }
                }
            }
            Ok(StateAck {
                succeeded: true,
                state: None,
                ..
            }) => {
                match self
                    .state
                    .remove_state(&self.interest.role, &self.interest.entity_name, &ce.key)
                    .await
                {
                    Ok(_) => {
                        trace!("Aggregate {self_id} state deleted.");
                        message.ack().await.ok();
                    }
                    Err(e) => {
                        error!("Failed to delete aggregate {self_id} state: {e}");
                        message.nack().await;
                    }
                }
            }
            Ok(StateAck {
                succeeded: false,
                error: Some(e),
                ..
            }) => {
                error!("Failed to apply event to target actor: {e}");
                message.nack().await;
            }
            Ok(StateAck {
                succeeded: false,
                error: None,
                ..
            }) => {
                error!("Failed to apply event to target actor - unspecified failure");
                message.nack().await;
            }
            Err(e) => {
                error!("Failed to apply event to target actor: {e}");
                message.nack().await;
            }
        }

        Ok(())
    }
}
