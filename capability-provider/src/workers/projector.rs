use async_nats::jetstream::{self, stream::Config, Context};
use cloudevents::Event as CloudEvent;
use tracing::debug;

use crate::{
    config::InterestDeclaration, consumers::WorkError, natsclient::AckableMessage,
    state::EntityState,
};

use crate::consumers::{WorkResult, Worker};

pub struct ProjectorEventWorker {
    pub nc: async_nats::Client,
    pub context: Context,
    pub interest: InterestDeclaration,
    pub state: EntityState,
}

impl ProjectorEventWorker {
    pub fn new(
        nc: async_nats::Client,
        context: Context,
        interest: InterestDeclaration,
        state: EntityState,
    ) -> Self {
        ProjectorEventWorker {
            nc,
            context,
            interest,
            state,
        }
    }
}

#[async_trait::async_trait]
impl Worker for ProjectorEventWorker {
    type Message = CloudEvent;

    async fn do_work(&self, mut message: AckableMessage<Self::Message>) -> WorkResult<()> {
        debug!(event = ?message.as_ref(), "Handling received event");
        // THIS IS WHERE WE'D DO REAL WORK
        message.ack().await.map_err(|e| WorkError::NatsError(e))?;

        Ok(())
    }
}
