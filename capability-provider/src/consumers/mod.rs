mod command_consumer;
mod command_worker;
mod event_consumer;
mod event_worker;
mod manager;

use std::{collections::HashMap, fmt::Debug, sync::Arc};

use async_nats::Error as NatsError;
pub use command_consumer::{CommandConsumer, RawCommand};
pub use command_worker::CommandWorker;
pub use event_consumer::EventConsumer;
pub use event_worker::EventWorker;
pub use manager::ConsumerManager;

use tokio::{sync::RwLock, task::JoinHandle};

use crate::{config::InterestDeclaration, natsclient::AckableMessage};

pub type WorkResult<T> = Result<T, WorkError>;
pub(crate) type WorkHandles = Arc<RwLock<HashMap<InterestDeclaration, JoinHandle<WorkResult<()>>>>>;

/// A helper trait to allow for constructing any consumer
#[async_trait::async_trait]
pub trait CreateConsumer {
    type Output: Unpin;

    /// Create a type of the specified `Output`
    async fn create(
        stream: async_nats::jetstream::stream::Stream,
        interest: InterestDeclaration,
    ) -> Result<Self::Output, NatsError>;
}

#[async_trait::async_trait]
pub trait Worker {
    /// The actual message type to expect, such as a cloud event or a command
    type Message: Debug + Send;
    /// Process the given work to completion. Almost all errors returned are things that could be
    /// retried. But if for some reason a fatal error occurs, return `WorkError::Fatal` to indicate
    /// that work should stop.
    async fn do_work(&self, message: AckableMessage<Self::Message>) -> WorkResult<()>;
}

/// An error that describes possible work failures when performing actions based on incoming messages
#[derive(Debug)]
pub enum WorkError {
    /// A consumer has stopped returning work in its stream and should be restarted    
    ConsumerStopped,
    /// A fatal error, generally returned by a [`Worker`] if it experiences some sort of failure it
    /// can't recover from. Should include the underlying error that caused the failure    
    Fatal(Box<dyn std::error::Error + Send>),
    /// An error occured when interacting with NATS    
    NatsError(async_nats::Error),
    /// A catch all error for non-described errors that are not fatal    
    Other(String),
}
