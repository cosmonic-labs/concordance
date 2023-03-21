mod command_consumer;
pub(crate) mod event_consumer;
mod event_worker;
mod manager;

use std::{collections::HashMap, fmt::Debug, sync::Arc};

use async_nats::Error as NatsError;
pub use command_consumer::{CommandConsumer, RawCommand};
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

/// Creates a futures::Stream for the given type, pulling items of the specified type by deserializing
/// them from JSON. The consumer types need to expose a `sanitize_type_name` function that is called
/// by each stream prior to delivering the message
macro_rules! impl_Stream {
    ($($t:ty; $u:ty),+) => {
        $(impl Stream for $t {
            type Item = Result<AckableMessage<$u>, NatsError>;

            fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                match self.stream.try_poll_next_unpin(cx) {
                    Poll::Ready(None) => Poll::Ready(None),
                    Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
                    Poll::Ready(Some(Ok(msg))) => {
                        // Convert to our $u type, skipping if we can't do it (and looping around to
                        // try the next poll)
                        let item: $u = match serde_json::from_slice(&msg.payload) {
                            Ok(item) => item,
                            Err(e) => {
                                warn!(error = ?e, "Unable to decode as <$u>. Skipping message");
                                let waker = cx.waker().clone();
                                tokio::spawn(async move {
                                    if let Err(e) = msg.ack().await {
                                        error!(error = %e, "Error when trying to ack skipped message, message will be redelivered")
                                    }
                                    waker.wake();
                                });
                                // Return a poll pending. It will then wake up and try again once it has acked
                                return Poll::Pending;
                            }
                        };
                        let item = <$t>::sanitize_type_name(item);
                        // NOTE(thomastaylor312): Ideally we'd consume `msg.payload` above with a
                        // `Cursor` and `from_reader` and then manually reconstruct the acking using the
                        // message context, but I didn't want to waste time optimizing yet
                        Poll::Ready(Some(Ok(AckableMessage {
                            inner: item,
                            acker: Some(msg),
                        })))
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
        })*
    }
}

pub(crate) use impl_Stream;
