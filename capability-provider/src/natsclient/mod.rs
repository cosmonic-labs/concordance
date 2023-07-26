mod natsconn;

const EVENT_STREAM_NAME: &str = "CC_EVENTS";
const EVENTS_STREAM_TOPIC: &str = "cc.events.*";

const COMMANDS_STREAM_NAME: &str = "CC_COMMANDS";
const COMMANDS_STREAM_TOPIC: &str = "cc.commands.*";

/// The default time given for an event/command to ack. Set to 3 to give a buffer
/// for actors that have a default timeout of 2s
pub(crate) const DEFAULT_ACK_TIME: std::time::Duration = std::time::Duration::from_secs(3);

// Timeout accounts for time to send message
pub(crate) const SEND_TIMEOUT_DURATION: tokio::time::Duration = tokio::time::Duration::from_secs(2);

use async_nats::{jetstream::AckKind, Error as NatsError};
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;
use std::time::Duration;

use tracing::{error, warn};

pub(crate) use natsconn::NatsClient;

pub struct AckableMessage<T> {
    pub(crate) inner: T,
    // Wrapped in an option so we only do it once
    pub(crate) acker: Option<async_nats::jetstream::Message>,
}

impl<T> AckableMessage<T> {
    /// Acks this message. This should be called when all work related to this message has been
    /// completed. If this is called before work is done (e.g. like sending a command), instability
    /// could occur. Calling this function again (or after nacking) is a noop.
    ///
    /// This function will only error after it has tried up to 3 times to ack the request. If it
    /// doesn't receive a response after those 3 times, this will return an error.
    pub async fn ack(&mut self) -> Result<(), NatsError> {
        // We want to double ack so we are sure that the server has marked this task as done
        if let Some(msg) = self.acker.take() {
            // Starting at 1 for humans/logging
            let mut retry_count = 1;
            loop {
                match msg.double_ack().await {
                    Ok(_) => break Ok(()),
                    Err(e) if retry_count == 3 => break Err(e),
                    Err(e) => {
                        warn!(error = %e, %retry_count, "Failed to receive ack response, will retry");
                        retry_count += 1;
                        tokio::time::sleep(Duration::from_millis(100)).await
                    }
                }
            }
        } else {
            warn!("Ack has already been sent");
            Ok(())
        }
    }

    pub async fn nack(&mut self) {
        if let Err(e) = self.custom_ack(AckKind::Nak(None)).await {
            error!(error = %e, "Error when nacking message");
            self.acker = None;
        }
    }

    async fn custom_ack(&mut self, kind: AckKind) -> Result<(), NatsError> {
        if let Some(msg) = self.acker.take() {
            if let Err(e) = msg.ack_with(kind).await {
                // Put it back so it can be called again
                self.acker = Some(msg);
                Err(e)
            } else {
                Ok(())
            }
        } else {
            warn!("Nack has already been sent");
            Ok(())
        }
    }
}

impl<T> Drop for AckableMessage<T> {
    fn drop(&mut self) {
        // self.nack escapes current lifetime, so just manually take the message
        if let Some(msg) = self.acker.take() {
            if let Ok(handle) = tokio::runtime::Handle::try_current() {
                handle.spawn(async move {
                    if let Err(e) = msg.ack_with(AckKind::Nak(None)).await {
                        warn!(error = %e, "Error when sending nack during drop")
                    }
                });
            } else {
                warn!("Couldn't find async runtime to send nack during drop")
            }
        }
    }
}

impl<T> AsRef<T> for AckableMessage<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> AsMut<T> for AckableMessage<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> Deref for AckableMessage<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for AckableMessage<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Debug> Debug for AckableMessage<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AckableMessage")
            .field("inner", &self.inner)
            .finish()
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::{COMMANDS_STREAM_NAME, EVENT_STREAM_NAME};
    use crate::{consumers::RawCommand, state::STATE_BUCKET_NAME, Result};

    pub(crate) async fn create_js_context() -> async_nats::jetstream::Context {
        let nc = async_nats::connect("127.0.0.1").await.unwrap();
        async_nats::jetstream::new(nc.clone())
    }

    pub(crate) async fn clear_streams(js: async_nats::jetstream::Context) {
        js.delete_stream(EVENT_STREAM_NAME).await.ok();
        js.delete_stream(COMMANDS_STREAM_NAME).await.ok();
        js.delete_key_value(STATE_BUCKET_NAME).await.ok();
    }

    pub(crate) async fn publish_command(
        client: &async_nats::Client,
        entity_name: &str,
        cmd: &RawCommand,
    ) -> Result<()> {
        let raw = serde_json::to_vec(&cmd).unwrap().into();
        // e.g. cc.commands.order or cc.commands.bankaccount
        client
            .publish(format!("cc.commands.{entity_name}"), raw)
            .await
            .unwrap();
        client.flush().await.unwrap();
        Ok(())
    }

    pub(crate) async fn publish_event(
        client: &async_nats::Client,
        event_type: &str,
        raw: &str,
    ) -> Result<()> {
        let raw = raw.as_bytes().to_vec().into();
        client
            .publish(format!("cc.events.{event_type}"), raw)
            .await
            .unwrap();
        client.flush().await.unwrap();
        Ok(())
    }
}
