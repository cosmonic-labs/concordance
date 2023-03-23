use crate::config::InterestDeclaration;

use cloudevents::{AttributesWriter, Event as CloudEvent};
use std::convert::TryFrom;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use crate::natsclient::{AckableMessage, DEFAULT_ACK_TIME};
use async_nats::{
    jetstream::{
        consumer::pull::{Config as PullConfig, Stream as MessageStream},
        stream::Stream as JsStream,
    },
    Error as NatsError,
};
use case::CaseExt;
use cloudevents::AttributesReader;
use futures::{Stream, TryStreamExt};
use tracing::{error, warn};

use super::{impl_Stream, CreateConsumer};

#[allow(dead_code)]
pub struct EventConsumer {
    stream: MessageStream,
    interest: InterestDeclaration,
    name: String,
}

impl EventConsumer {
    pub fn sanitize_type_name(evt: CloudEvent) -> CloudEvent {
        let mut event = evt.clone();
        event.set_type(evt.ty().to_string().to_snake());
        event
    }

    pub async fn try_new(
        stream: JsStream,
        interest: InterestDeclaration,
    ) -> ::std::result::Result<EventConsumer, NatsError> {
        let consumer_name = interest.consumer_name();
        let friendly_name = interest.to_string();

        let consumer = stream
            .get_or_create_consumer(
                &consumer_name,
                PullConfig {
                    durable_name: Some(consumer_name.clone()),
                    name: Some(consumer_name.clone()),
                    description: Some(format!("Durable event consumer for {friendly_name}")),
                    ack_policy: async_nats::jetstream::consumer::AckPolicy::Explicit,
                    ack_wait: DEFAULT_ACK_TIME,
                    // poison pill identified after 3 nacks
                    max_deliver: 3,
                    deliver_policy: async_nats::jetstream::consumer::DeliverPolicy::All,
                    // TODO: when NATS server and async nats client support it, convert this
                    // to declare explicit per-event interest rather than subscribing to all
                    //filter_subject: "cc.events.a,cc.events.b,etc".to_string(),
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
        Ok(EventConsumer {
            stream: messages,
            interest,
            name: info.name.to_string(),
        })
    }
}

// Creates a futures::Stream for EventConsumer, pulling items of type CloudEvent
impl_Stream!(EventConsumer; CloudEvent);

#[async_trait::async_trait]
impl CreateConsumer for EventConsumer {
    type Output = EventConsumer;

    async fn create(
        stream: async_nats::jetstream::stream::Stream,
        interest: InterestDeclaration,
    ) -> Result<Self::Output, NatsError> {
        EventConsumer::try_new(stream, interest).await
    }
}

#[cfg(test)]
pub(crate) mod test {
    use cloudevents::{Data, Event as CloudEvent};
    use futures::{Stream, TryStreamExt};
    use tokio::time::timeout;
    use wasmbus_rpc::core::LinkDefinition;

    use crate::{
        config::InterestDeclaration,
        consumers::EventConsumer,
        natsclient::{
            test::create_js_context,
            test::{clear_streams, publish_event},
            AckableMessage, NatsClient, SEND_TIMEOUT_DURATION,
        },
    };

    pub const EVENT1: &str = r##"
    {
        "data": {
            "amount": 200,
            "tx_type": "withdrawal"            
        },
        "datacontenttype": "application/json",
        "id": "5613ee03-8645-4ad8-a3aa-3428a33b7e96",
        "source": "testy mctesto",
        "specversion": "1.0",
        "time": "2023-02-14T19:21:09.011785Z",
        "type": "amount_withdrawn"
    }
"##;
    pub const EVENT2: &str = r##"
    {
        "data": {
            "amount": 100,
            "tx_type": "deposit"            
        },
        "datacontenttype": "application/json",
        "id": "5613ee03-8645-4ad8-a3aa-3428a33b7e97",
        "source": "testy mctesto",
        "specversion": "1.0",
        "time": "2023-02-14T19:21:14.00Z",
        "type": "amount_deposited"
    }
"##;

    #[tokio::test]
    async fn event_consumer_pulls_messages() {
        let nc = async_nats::connect("127.0.0.1").await.unwrap();
        let js = create_js_context().await;
        clear_streams(js.clone()).await;

        let client = NatsClient::new(nc.clone(), js.clone());
        let (e, _c) = client.ensure_streams().await.unwrap();

        let agg = InterestDeclaration::aggregate_for_events(
            "Mxbob",
            "order",
            "order_id",
            LinkDefinition::default(),
        );
        let mut ec = EventConsumer::try_new(e, agg).await.unwrap();

        publish_event(&nc, "amount_withdrawn", EVENT1)
            .await
            .unwrap();
        publish_event(&nc, "amount_deposited", EVENT2)
            .await
            .unwrap();

        let mut evt = wait_for_event(&mut ec).await;
        let data = evt.data().cloned().unwrap();
        if let Data::Json(j) = data {
            assert_eq!(j["amount"], 200);
            assert_eq!(j["tx_type"], "withdrawal");
        } else {
            assert!(false);
        }

        evt.ack().await.expect("Should be able to ack message");

        let mut evt = wait_for_event(&mut ec).await;
        let data = evt.data().cloned().unwrap();
        if let Data::Json(j) = data {
            assert_eq!(j["amount"], 100);
            assert_eq!(j["tx_type"], "deposit");
        } else {
            assert!(false);
        }
        evt.ack().await.expect("Should be able to ack message");

        clear_streams(js.clone()).await;
    }

    #[tokio::test]
    async fn nack_and_rereceive() {
        //TODO
        assert!(true);
    }

    pub async fn wait_for_event(
        mut stream: impl Stream<Item = Result<AckableMessage<CloudEvent>, async_nats::Error>> + Unpin,
    ) -> AckableMessage<CloudEvent> {
        timeout(SEND_TIMEOUT_DURATION, stream.try_next())
            .await
            .expect("Should have received event before timeout")
            .expect("Stream shouldn't have had an error")
            .expect("Stream shouldn't have ended")
    }
}
