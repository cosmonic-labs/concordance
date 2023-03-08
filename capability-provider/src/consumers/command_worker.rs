use async_nats::jetstream::Context;
use tracing::debug;

use crate::{
    config::InterestDeclaration, consumers::WorkError, natsclient::AckableMessage,
    state::EntityState,
};

use super::{RawCommand, WorkResult, Worker};

pub struct CommandWorker {
    pub context: Context,
    pub interest: InterestDeclaration,
    pub state: EntityState,
}

impl CommandWorker {
    pub fn new(context: Context, interest: InterestDeclaration, state: EntityState) -> Self {
        CommandWorker {
            context,
            interest,
            state,
        }
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
