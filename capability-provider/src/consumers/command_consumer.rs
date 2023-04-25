use crate::config::{ActorRole, InterestDeclaration};
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

use crate::natsclient::{AckableMessage, DEFAULT_ACK_TIME};

use super::{impl_Stream, CreateConsumer};

/// The JSON command as pulled off of the stream by way of a command consumer
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawCommand {
    pub command_type: String,
    pub key: String,
    pub data: serde_json::Value,
}

pub struct CommandConsumer {
    stream: MessageStream,
}

impl CommandConsumer {
    pub fn sanitize_type_name(cmd: RawCommand) -> RawCommand {
        RawCommand {
            command_type: cmd.command_type.to_snake(),
            ..cmd
        }
    }

    pub async fn try_new(
        stream: JsStream,
        interest: InterestDeclaration,
    ) -> Result<CommandConsumer, NatsError> {
        let consumer_name = interest.consumer_name();
        if interest.role != ActorRole::Aggregate {
            return Err(format!(
                "Only aggregates are allowed to receive commands, supplied role was {}",
                interest.role
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

        let messages = consumer
            .stream()
            .max_messages_per_batch(1)
            .messages()
            .await?;
        Ok(CommandConsumer { stream: messages })
    }
}

// Creates a futures::Stream for CommandConsumer, pulling items of type RawCommand
impl_Stream!(CommandConsumer; RawCommand);

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
    use wasmbus_rpc::core::LinkDefinition;

    use crate::{
        config::{ActorInterest, ActorRole, InterestConstraint, InterestDeclaration},
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

        let client = NatsClient::new(js.clone());
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
        let agg = InterestDeclaration::aggregate_for_commands(
            "Mxbob",
            "order",
            "order_id",
            LinkDefinition::default(),
        );
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
        let js = create_js_context().await;
        let client = NatsClient::new(js.clone());
        let (_e, c) = client.ensure_streams().await.unwrap();
        let not_an_aggregate = InterestDeclaration {
            actor_id: "bob".to_string(),
            entity_name: "testbob".to_string(),
            key_field: "order_id".to_string(),
            interest_constraint: InterestConstraint::Events,
            interest: ActorInterest::None,
            role: ActorRole::Projector,
            link_definition: LinkDefinition::default(),
        };
        clear_streams(js).await;

        let cc = CommandConsumer::try_new(c, not_an_aggregate).await;
        assert!(cc.is_err());
    }

    #[tokio::test]
    async fn command_consumer_creates_on_happy_path() {
        let js = create_js_context().await;
        let client = NatsClient::new(js.clone());
        let (_e, c) = client.ensure_streams().await.unwrap();

        let agg = InterestDeclaration::aggregate_for_commands(
            "Mxbob",
            "superbob",
            "order_id",
            LinkDefinition::default(),
        );
        let cc = CommandConsumer::try_new(c, agg).await;
        assert!(cc.is_ok());

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
