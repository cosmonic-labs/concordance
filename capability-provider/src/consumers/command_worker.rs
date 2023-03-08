use async_nats::jetstream::Context;
use tracing::debug;

use crate::{consumers::WorkError, natsclient::AckableMessage};

use super::{RawCommand, WorkResult, Worker};

pub struct CommandWorker {
    pub context: Context,
}

impl CommandWorker {
    pub fn new(context: Context) -> Self {
        CommandWorker { context }
    }
}

#[async_trait::async_trait]
impl Worker for CommandWorker {
    type Message = RawCommand;

    async fn do_work(&self, mut message: AckableMessage<Self::Message>) -> WorkResult<()> {
        println!("HELLO");
        debug!(command = ?message.as_ref(), "Handling received command");
        // THIS IS WHERE WE'D DO REAL WORK
        message.ack().await.map_err(|e| WorkError::NatsError(e))?;

        Ok(())
    }
}
