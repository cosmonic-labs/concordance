use crate::{
    natsclient::{
        COMMANDS_STREAM_NAME, COMMANDS_STREAM_TOPIC, EVENTS_STREAM_TOPIC, EVENT_STREAM_NAME,
    },
    Result,
};
use async_nats::jetstream::stream::{Config as StreamConfig, Stream};
use tracing::{debug, instrument};
use wasmbus_rpc::error::RpcError;

pub(crate) struct NatsClient {
    context: async_nats::jetstream::Context,
}

impl NatsClient {
    pub fn new(js: async_nats::jetstream::Context) -> NatsClient {
        NatsClient { context: js }
    }

    #[instrument(level = "debug", skip(self))]
    pub async fn ensure_streams(&self) -> Result<(Stream, Stream)> {
        let event_stream = self
            .context
            .get_or_create_stream(StreamConfig {
                name: EVENT_STREAM_NAME.to_string(),
                description: Some(
                    "Concordance event stream for event sourcing capability provider".to_string(),
                ),
                num_replicas: 1,
                retention: async_nats::jetstream::stream::RetentionPolicy::Limits, // does not delete upon ack, overlapping interest consumers ARE allowed
                subjects: vec![EVENTS_STREAM_TOPIC.to_owned()],
                storage: async_nats::jetstream::stream::StorageType::File,
                allow_rollup: false,
                ..Default::default()
            })
            .await
            .map_err(|e| RpcError::Nats(format!("{e:?}")))?;

        let command_stream = self
            .context
            .get_or_create_stream(StreamConfig {
                name: COMMANDS_STREAM_NAME.to_string(),
                description: Some(
                    "Concordance command stream for event sourcing capability provider".to_string(),
                ),
                num_replicas: 1,
                retention: async_nats::jetstream::stream::RetentionPolicy::WorkQueue, // delete upon ack, overlapping interest consumers ARE NOT allowed
                subjects: vec![COMMANDS_STREAM_TOPIC.to_owned()],
                storage: async_nats::jetstream::stream::StorageType::File,
                allow_rollup: false,
                ..Default::default()
            })
            .await
            .map_err(|e| RpcError::Nats(format!("{e:?}")))?;

        debug!("Detected or created both CC_EVENTS and CC_COMMANDS");

        Ok((event_stream, command_stream))
    }
}

#[cfg(test)]
mod test {
    use crate::natsclient::{COMMANDS_STREAM_NAME, EVENT_STREAM_NAME};

    use super::NatsClient;

    #[tokio::test]
    async fn test_ensure_streams() {
        let nc = async_nats::connect("127.0.0.1").await.unwrap();
        let js = async_nats::jetstream::new(nc.clone());
        let nc = NatsClient::new(js.clone());

        let (a, b) = nc.ensure_streams().await.unwrap();
        let (c, d) = nc.ensure_streams().await.unwrap(); // idempotency check

        assert_eq!(a.cached_info().config.name, c.cached_info().config.name);
        assert_eq!(b.cached_info().config.name, d.cached_info().config.name);

        js.delete_stream(EVENT_STREAM_NAME).await.unwrap();
        js.delete_stream(COMMANDS_STREAM_NAME).await.unwrap();

        assert!(true);
    }
}
