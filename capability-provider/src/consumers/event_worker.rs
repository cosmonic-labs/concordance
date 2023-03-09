use async_nats::jetstream::{self, stream::Config, Context};
use cloudevents::Event as CloudEvent;
use tracing::debug;

use crate::{
    config::InterestDeclaration, consumers::WorkError, natsclient::AckableMessage,
    state::EntityState,
};

use super::{WorkResult, Worker};

pub struct EventWorker {
    pub nc: async_nats::Client,
    pub context: Context,
    pub interest: InterestDeclaration,
    pub state: EntityState,
}

impl EventWorker {
    pub fn new(
        nc: async_nats::Client,
        context: Context,
        interest: InterestDeclaration,
        state: EntityState,
    ) -> Self {
        EventWorker {
            nc,
            context,
            interest,
            state,
        }
    }
}

#[async_trait::async_trait]
impl Worker for EventWorker {
    type Message = CloudEvent;

    async fn do_work(&self, mut message: AckableMessage<Self::Message>) -> WorkResult<()> {
        debug!(event = ?message.as_ref(), "Handling received event");
        // THIS IS WHERE WE'D DO REAL WORK
        message.ack().await.map_err(|e| WorkError::NatsError(e))?;

        Ok(())
    }
}
