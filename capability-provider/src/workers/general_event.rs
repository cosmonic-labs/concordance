use async_nats::jetstream::Context;
use cloudevents::Event as CloudEvent;
use tracing::{debug, error, instrument, trace};

use crate::eventsourcing::{
    Event as ConcordanceEvent, StatelessEventHandlerService, StatelessEventHandlerServiceSender,
};
use crate::{
    config::InterestDeclaration, consumers::WorkError, natsclient::AckableMessage,
    state::EntityState,
};

use crate::consumers::{WorkResult, Worker};

pub struct GeneralEventWorker {
    pub nc: async_nats::Client,
    pub context: Context,
    pub interest: InterestDeclaration,
    pub state: EntityState,
}

impl GeneralEventWorker {
    pub fn new(
        nc: async_nats::Client,
        context: Context,
        interest: InterestDeclaration,
        state: EntityState,
    ) -> Self {
        GeneralEventWorker {
            nc,
            context,
            interest,
            state,
        }
    }
}

#[async_trait::async_trait]
impl Worker for GeneralEventWorker {
    type Message = CloudEvent;

    /// Performs the work necessary to process an incoming concordance event and deliver it to the target actor
    /// nacks upon actor failure, acks upon actor success. No state is managed by the provider on behalf of
    /// general event workers
    #[instrument(level = "debug", skip_all, fields(actor_id = self.interest.actor_id))]
    async fn do_work(&self, mut message: AckableMessage<Self::Message>) -> WorkResult<()> {
        debug!(event = ?message.as_ref(), "Handling received event");

        // note the 'into' here converts from CloudEvent to ConcordanceEvent
        let ce: ConcordanceEvent = message.inner.clone().into();
        let self_id = self.interest.actor_id.to_string();

        if !self.interest.is_interested_in_event(&ce) {
            trace!(
                "General event handler is not interested in event '{}' on stream '{}'. Acking and moving on.",
                ce.event_type,
                ce.stream
            );
            message.ack().await.map_err(|e| WorkError::NatsError(e))?;
            return Ok(());
        }

        let ctx = wasmbus_rpc::provider::prelude::Context::default();
        let target = StatelessEventHandlerServiceSender::for_actor(&self.interest.link_definition);
        match target.apply_stateless_event(&ctx, &ce).await {
            Ok(_) => {
                message.ack().await.map_err(|e| WorkError::NatsError(e))?;
            }
            Err(e) => {
                error!("Failed to apply event to general event handler {self_id}: {e}");
                message.nack().await;
            }
        };

        Ok(())
    }
}
