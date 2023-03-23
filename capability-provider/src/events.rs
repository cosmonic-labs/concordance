use crate::Result;
use crate::{consumers::RawCommand, eventsourcing::Event as ConcordanceEvent};
use case::CaseExt;
use chrono::Utc; // only using chrono because cloudevents SDK needs it
use cloudevents::AttributesReader;
use tracing::{error, instrument};
use wasmbus_rpc::error::RpcError;

use cloudevents::{Event as CloudEvent, EventBuilder, EventBuilderV10};

pub(crate) const EVENT_TOPIC_PREFIX: &str = "cc.events";
pub(crate) const COMMAND_TOPIC_PREFIX: &str = "cc.commands";
// pub(crate) const EXT_CONCORDANCE_AGGREGATE_KEY: &str = "x-concordance-agg-key";
// pub(crate) const EXT_CONCORDANCE_PM_KEY: &str = "x-concordance-pm-key";
pub(crate) const EXT_CONCORDANCE_STREAM: &str = "x-concordance-stream";

// NOTE: making the publication functions below use request versus publish forces
// the stream to acknowledge the new entry. Un-acked messages will result in errors

#[instrument(level = "debug", skip(nc))]
pub(crate) async fn publish_es_event(
    nc: &async_nats::Client,
    event: ConcordanceEvent,
) -> Result<()> {
    let evt_type = event.event_type.to_snake();
    let topic = format!("{EVENT_TOPIC_PREFIX}.{evt_type}");

    let cloud_event: CloudEvent = event.into();
    let Ok(raw) = serde_json::to_vec(&cloud_event) else {
        error!("Failed to serialize a stock cloudevent. Something is very wrong.");
        return Err(RpcError::Ser("Fatal serialization failure - could not serialize a cloud event".to_string()));
    };

    nc.request(topic, raw.into())
        .await
        .map_err(|e| RpcError::Nats(e.to_string()))?;

    Ok(())
}

#[instrument(level = "debug", skip(nc))]
pub(crate) async fn publish_raw_command(nc: &async_nats::Client, cmd: RawCommand) -> Result<()> {
    let cmd_type = cmd.command_type.to_snake();
    let topic = format!("{COMMAND_TOPIC_PREFIX}.{cmd_type}");

    let Ok(raw) = serde_json::to_vec(&cmd) else {
        error!("Failed to serialize an internal raw command. Something is very wrong.");
        return Err(RpcError::Ser("Fatal serialization failure - could not serialize a raw command".to_string()));
    };

    nc.request(topic, raw.into())
        .await
        .map_err(|e| RpcError::Nats(e.to_string()))?;

    Ok(())
}

/// Converts an internal Concordance Event (defined by interface IDL) into a cloud event. This strips the intermediary
/// envelope from the concordance event type to create a nice and tidy cloud event with JSON payload. It takes the
/// previously enveloped values of key and stream and places them on the cloud event by way of extensions
impl Into<CloudEvent> for ConcordanceEvent {
    fn into(self) -> CloudEvent {
        let mut evt = EventBuilderV10::new()
            .id(uuid::Uuid::new_v4().to_string())
            .ty(self.event_type.to_string())
            .source("concordance")
            .time(Utc::now())
            // .extension(EXT_CONCORDANCE_AGGREGATE_KEY, self.aggregate_key)
            // .extension(EXT_CONCORDANCE_PM_KEY, self.pm_key)
            .extension(EXT_CONCORDANCE_STREAM, self.stream)
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

/// Creates an internal Concordance Event (defined by interface IDL) from a cloud event. This will reconstitute the
/// intermediary envelope of the event and put the cloud event's JSON `data()` field into the `payload` field. The
/// key and stream values of the concordance event will be pulled from the appropriate extension fields on the cloud event
impl Into<ConcordanceEvent> for CloudEvent {
    fn into(self) -> ConcordanceEvent {
        let payload = match self.data() {
            Some(cloudevents::Data::Json(j)) => serde_json::to_vec(&j).unwrap_or_else(|_| vec![]),
            _ => {
                vec![]
            }
        };
        ConcordanceEvent {
            event_type: self.ty().to_owned(),
            // aggregate_key: self
            //     .extension(EXT_CONCORDANCE_AGGREGATE_KEY)
            //     .cloned()
            //     .unwrap_or("".to_string().into())
            //     .to_string(),
            // pm_key: self
            //     .extension(EXT_CONCORDANCE_PM_KEY)
            //     .cloned()
            //     .unwrap_or("".to_string().into())
            //     .to_string(),
            stream: self
                .extension(EXT_CONCORDANCE_STREAM)
                .cloned()
                .unwrap_or("".to_string().into())
                .to_string(),
            payload,
        }
    }
}

#[cfg(test)]
mod test {
    use cloudevents::{event::ExtensionValue, Data};
    use serde::{Deserialize, Serialize};

    use super::CloudEvent;
    use crate::{events::EXT_CONCORDANCE_STREAM, eventsourcing::Event as ConcordanceEvent};

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
    fn to_from_roundtrip() {
        let ace = AccountCreatedEvent {
            account_number: "ABC123".to_string(),
            min_balance: 1000,
            initial_balance: 100,
        };

        let internal_event = ConcordanceEvent {
            event_type: "account_created".to_string(),
            // aggregate_key: "ABC123".to_string(),
            // pm_key: "".to_string(),
            payload: serde_json::to_vec(&ace).unwrap(),
            stream: "bankaccount".to_string(),
        };
        let ce: CloudEvent = internal_event.into();
        // assert_eq!(
        //     ce.extension(EXT_CONCORDANCE_AGGREGATE_KEY),
        //     Some(&ExtensionValue::String("ABC123".to_string()))
        // );
        assert_eq!(
            ce.extension(EXT_CONCORDANCE_STREAM),
            Some(&ExtensionValue::String("bankaccount".to_string()))
        );

        // Ensure that we can get the original strongly-typed event out of the cloud event's `data` field
        if let Some(Data::Json(j)) = ce.data() {
            let round_tripped: AccountCreatedEvent = serde_json::from_value(j.to_owned()).unwrap();
            assert_eq!(round_tripped.account_number, "ABC123");
            assert_eq!(round_tripped.initial_balance, 100);
        }

        let ie2: ConcordanceEvent = ce.into();
        assert_eq!(ie2.event_type, "account_created");
        assert_eq!(ie2.stream, "bankaccount");
        let ace2: AccountCreatedEvent = serde_json::from_slice(&ie2.payload).unwrap();
        assert_eq!(ace2.account_number, "ABC123");
        assert_eq!(ace2.min_balance, 1000);
    }
}
