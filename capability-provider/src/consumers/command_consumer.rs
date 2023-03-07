use async_nats::{
    jetstream::{
        consumer::pull::{Config as PullConfig, Stream as MessageStream},
        stream::Stream as JsStream,
    },
    Error as NatsError,
};
use case::CaseExt;
use futures::{Stream, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::task::{Context, Poll};
use tracing::{error, warn};

use crate::config::{ActorRole, InterestDeclaration};

use crate::natsclient::{AckableMessage, DEFAULT_ACK_TIME};

use super::CreateConsumer;

/// The JSON command as pulled off of the stream by way of a command consumer
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawCommand {
    pub command_type: String,
    pub key: String,
    pub data: serde_json::Value,
}

impl RawCommand {
    pub fn sanitize_typename(self) -> RawCommand {
        RawCommand {
            command_type: self.command_type.to_snake(),
            ..self
        }
    }
}

pub struct CommandConsumer {
    stream: MessageStream,
    interest: InterestDeclaration,
    name: String,
    topic: String,
}

impl CommandConsumer {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn topic(&self) -> String {
        self.topic.clone()
    }

    pub async fn try_new(
        stream: JsStream,
        interest: InterestDeclaration,
    ) -> Result<CommandConsumer, NatsError> {
        let consumer_name = interest.consumer_name();
        if interest.role != ActorRole::Aggregate {
            return Err(format!(
                "Only aggregates are allowed to receive commands, supplied role was {}",
                interest.role.to_string()
            )
            .into());
        }
        let friendly_name = interest.to_string();
        let agg_name = interest.entity_name.clone();

        let consumer = stream
            .get_or_create_consumer(
                &consumer_name,
                PullConfig {
                    durable_name: Some(consumer_name.clone()),
                    name: Some(consumer_name.clone()),
                    description: Some(format!("Durable command consumer for {friendly_name}")),
                    ack_policy: async_nats::jetstream::consumer::AckPolicy::Explicit,
                    ack_wait: DEFAULT_ACK_TIME,
                    // poison pill identified after 3 nacks
                    max_deliver: 3,
                    deliver_policy: async_nats::jetstream::consumer::DeliverPolicy::All,
                    filter_subject: format!("cc.commands.{agg_name}"),
                    ..Default::default()
                },
            )
            .await?;

        let info = consumer.cached_info();
        let messages = consumer
            .stream()
            .max_messages_per_batch(1)
            .messages()
            .await?;
        Ok(CommandConsumer {
            stream: messages,
            interest,
            name: info.name.to_string(),
            topic: info.config.filter_subject.to_string(),
        })
    }
}

impl Stream for CommandConsumer {
    type Item = Result<AckableMessage<RawCommand>, NatsError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.stream.try_poll_next_unpin(cx) {
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(Some(Ok(msg))) => {
                // Convert to our event type, skipping if we can't do it (and looping around to
                // try the next poll)
                let cmd: RawCommand = match serde_json::from_slice(&msg.payload) {
                    Ok(cmd) => cmd,
                    Err(e) => {
                        warn!(error = ?e, "Unable to decode as command. Skipping message");
                        // This is slightly janky, but rather than having to store and poll the
                        // future (which gets a little gnarly), just pass the message onto a
                        // spawned thread which wakes up the thread when it is done acking.
                        let waker = cx.waker().clone();
                        // NOTE: If we are already in a stream impl, we should be able to spawn
                        // without worrying. A panic isn't the worst here if for some reason we
                        // can't as it means we can't ack the message and we'll be stuck waiting
                        // for it to deliver again until it fails
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
                let cmd = cmd.sanitize_typename();
                // NOTE(thomastaylor312): Ideally we'd consume `msg.payload` above with a
                // `Cursor` and `from_reader` and then manually reconstruct the acking using the
                // message context, but I didn't want to waste time optimizing yet
                Poll::Ready(Some(Ok(AckableMessage {
                    inner: cmd,
                    acker: Some(msg),
                })))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[async_trait::async_trait]
impl CreateConsumer for CommandConsumer {
    type Output = CommandConsumer;

    async fn create(
        stream: async_nats::jetstream::stream::Stream,
        interest: InterestDeclaration,
    ) -> Result<Self::Output, NatsError> {
        CommandConsumer::try_new(stream, interest).await
    }
}

#[cfg(test)]
mod test {
    use futures::{Stream, TryStreamExt};
    use serde_json::json;
    use tokio::time::timeout;

    use crate::{
        config::{ActorInterest, ActorRole, InterestDeclaration},
        consumers::{CommandConsumer, RawCommand},
        natsclient::{
            test::create_js_context,
            test::{clear_streams, publish_command},
            AckableMessage, NatsClient, SEND_TIMEOUT_DURATION,
        },
    };

    #[tokio::test]
    async fn command_consumer_stream_pulls_messages() {
        let nc = async_nats::connect("127.0.0.1").await.unwrap();
        let js = create_js_context().await;
        clear_streams(js.clone()).await;

        let client = NatsClient::new(nc.clone(), js.clone());
        let (_e, c) = client.ensure_streams().await.unwrap();

        let cmds = vec![
            RawCommand {
                command_type: "test_one".to_string(),
                key: "alfred".to_string(),
                data: json!({
                    "hello": "world"
                }),
            },
            RawCommand {
                command_type: "test_two".to_string(),
                key: "alfred2".to_string(),
                data: json!({
                    "hello": "world2"
                }),
            },
            RawCommand {
                command_type: "test_three".to_string(),
                key: "alfred3".to_string(),
                data: json!({
                    "hello": "world3"
                }),
            },
        ];

        // the "order" aggregate to consume here, e.g. AGG_order
        let agg = InterestDeclaration::aggregate("Mxbob", "order");
        let mut cc = CommandConsumer::try_new(c, agg).await.unwrap();

        let c = nc.clone();
        for cmd in cmds {
            publish_command(&c, "order", &cmd).await.unwrap();
        }

        let mut cmd = wait_for_command(&mut cc).await;
        assert_eq!(cmd.key, "alfred");
        assert_eq!(cmd.command_type, "test_one");

        cmd.ack().await.expect("Should be able to ack message");

        let mut cmd = wait_for_command(&mut cc).await;
        assert_eq!(cmd.key, "alfred2");
        assert_eq!(cmd.command_type, "test_two");

        cmd.ack().await.expect("Should be able to ack message");

        let mut cmd = wait_for_command(&mut cc).await;
        assert_eq!(cmd.key, "alfred3");
        assert_eq!(cmd.command_type, "test_three");

        cmd.ack().await.expect("Should be able to ack message");

        clear_streams(js).await;
    }

    #[tokio::test]
    async fn command_consumer_fails_for_non_agg() {
        let nc = async_nats::connect("127.0.0.1").await.unwrap();
        let js = create_js_context().await;
        let client = NatsClient::new(nc, js.clone());
        let (_e, c) = client.ensure_streams().await.unwrap();
        let not_an_aggregate = InterestDeclaration {
            actor_id: "bob".to_string(),
            entity_name: "testbob".to_string(),
            interest: ActorInterest::None,
            role: ActorRole::Projector,
        };
        clear_streams(js).await;

        let cc = CommandConsumer::try_new(c, not_an_aggregate).await;
        assert!(cc.is_err());
    }

    #[tokio::test]
    async fn command_consumer_creates_on_happy_path() {
        let nc = async_nats::connect("127.0.0.1").await.unwrap();
        let js = create_js_context().await;
        let client = NatsClient::new(nc, js.clone());
        let (_e, c) = client.ensure_streams().await.unwrap();

        let agg = InterestDeclaration::aggregate("Mxbob", "superbob");
        let cc = CommandConsumer::try_new(c, agg).await;
        assert!(cc.is_ok());

        let cc = cc.unwrap();
        assert_eq!(cc.name(), "AGG_superbob");
        assert_eq!(cc.topic(), "cc.commands.superbob");

        clear_streams(js).await;
    }

    async fn wait_for_command(
        mut stream: impl Stream<Item = Result<AckableMessage<RawCommand>, async_nats::Error>> + Unpin,
    ) -> AckableMessage<RawCommand> {
        timeout(SEND_TIMEOUT_DURATION, stream.try_next())
            .await
            .expect("Should have received command before timeout")
            .expect("Stream shouldn't have had an error")
            .expect("Stream shouldn't have ended")
    }
}
