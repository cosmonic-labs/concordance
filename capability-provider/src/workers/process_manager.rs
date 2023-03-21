use async_nats::jetstream::Context;
use cloudevents::Event as CloudEvent;
use tracing::debug;

use crate::{
    config::InterestDeclaration,
    consumers::{WorkError, WorkResult, Worker},
    events::publish_es_event,
    eventsourcing::{AggregateService, AggregateServiceSender, StatefulCommand},
    natsclient::AckableMessage,
    state::EntityState,
};

pub struct ProcessManagerWorker {
    pub nc: async_nats::Client,
    pub context: Context,
    pub interest: InterestDeclaration,
    pub state: EntityState,
}

#[async_trait::async_trait]
impl Worker for ProcessManagerWorker {
    type Message = CloudEvent;

    async fn do_work(&self, mut message: AckableMessage<Self::Message>) -> WorkResult<()> {
        debug!(event = ?message.as_ref(), "Handling received event");
        // THIS IS WHERE WE'D DO REAL WORK
        message.ack().await.map_err(|e| WorkError::NatsError(e))?;

        Ok(())
    }
}
