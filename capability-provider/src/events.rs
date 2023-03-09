use crate::eventsourcing::Event;
use crate::Result;
use case::CaseExt;
use tracing::instrument;
use wasmbus_rpc::error::RpcError;

use cloudevents::{Event as CloudEvent, EventBuilder, EventBuilderV10};

const EVENT_TOPIC_PREFIX: &str = "cc.events";

#[instrument(level = "trace")]
pub(crate) async fn publish_es_event(nc: &async_nats::Client, event: Event) -> Result<()> {
    let evt_type = event.event_type.to_snake();
    let topic = format!("{EVENT_TOPIC_PREFIX}.{evt_type}");
    let ce: CloudEvent = event.into();
    let raw = serde_json::to_string(&ce).unwrap();
    let raw = raw.as_bytes().to_vec().into();

    nc.publish(topic, raw)
        .await
        .map_err(|e| RpcError::Nats(e.to_string()))?;

    Ok(())
}

impl Into<CloudEvent> for Event {
    fn into(self) -> CloudEvent {
        let mut evt = EventBuilderV10::new()
            .id(uuid::Uuid::new_v4().to_string())
            .ty(self.event_type.to_string())
            .source("concordance")
            .build()
            .unwrap(); // if we can't serialize this envelope, something's bad enough worth panicking for

        // FYI: `payload` was already run through serde_json by the actor that produced the Event
        evt.set_data(
            "application/json",
            serde_json::from_slice::<serde_json::Value>(&self.payload).unwrap(),
        );

        evt
    }
}

#[cfg(test)]
mod test {
    use cloudevents::Data;
    use serde::{Deserialize, Serialize};

    use super::CloudEvent;
    use crate::eventsourcing::Event;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct CreateAccountCommand {
        pub account_number: String,
        pub min_balance: u32,
        pub initial_balance: u32,
        pub customer_id: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct AccountCreatedEvent {
        pub initial_balance: u32,
        pub account_number: String,
        pub min_balance: u32,
    }

    #[test]
    fn cloudevent_round_trip() {
        let ca = CreateAccountCommand {
            account_number: "ABC123".to_string(),
            min_balance: 100,
            initial_balance: 500,
            customer_id: "BOB".to_string(),
        };

        // this is what an actor/aggregate would create
        let internal_event = Event {
            event_type: "account_created".to_string(),
            key: "ABC123".to_string(),
            payload: serde_json::to_vec(&ca).unwrap(),
            stream: "bankaccount".to_string(),
        };

        let mut ce: CloudEvent = internal_event.into(); // use our impl Into<..>
        let (_datacontenttype, _dataschema, data) = ce.take_data();
        if let Some(Data::Json(j)) = data {
            let round_tripped: AccountCreatedEvent = serde_json::from_value(j).unwrap();
            assert_eq!(round_tripped.account_number, "ABC123");
            assert_eq!(round_tripped.initial_balance, 500);
        } else {
            assert!(false);
        }
    }
}
