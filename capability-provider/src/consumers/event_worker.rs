use async_nats::jetstream::{self, stream::Config, Context};
use cloudevents::Event as CloudEvent;
use tracing::debug;

use crate::{consumers::WorkError, natsclient::AckableMessage};

use super::{WorkResult, Worker};

pub struct EventWorker {
    pub context: Context,
}

impl EventWorker {
    pub fn new(context: Context) -> Self {
        EventWorker { context }
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
