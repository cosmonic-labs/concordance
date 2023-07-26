use async_nats::jetstream::{kv::Config as KvConfig, kv::Store, Context};
use tracing::{error, instrument, trace};
use wasmbus_rpc::error::RpcError;

use crate::{config::ActorRole, Result};

pub(crate) const STATE_BUCKET_NAME: &str = "CC_STATE";

#[derive(Clone)]
pub struct EntityState {
    bucket: Store,
}

impl EntityState {
    pub async fn new_from_context(context: &async_nats::jetstream::Context) -> Result<EntityState> {
        Ok(EntityState {
            bucket: get_or_create_bucket(context).await?,
        })
    }

    #[instrument(level = "debug", skip(self, state))]
    pub async fn write_state(
        &self,
        actor_role: &ActorRole,
        entity_name: &str,
        key: &str,
        state: Vec<u8>,
    ) -> Result<()> {
        trace!("Writing state");

        let key = state_key(actor_role, entity_name, key);

        self.bucket
            .put(&key, state.into())
            .await
            .map_err(|err| {
                let err_msg = format!("Failed to write state @ {key}: {err:?}");
                error!(error = %err, message = err_msg);
                RpcError::Nats(err_msg)
            })
            .map(|_| ())
    }

    #[instrument(level = "debug", skip(self))]
    pub async fn fetch_state(
        &self,
        actor_role: &ActorRole,
        entity_name: &str,
        key: &str,
    ) -> Result<Option<Vec<u8>>> {
        trace!("Fetching state");
        let key = state_key(actor_role, entity_name, key);

        self.bucket
            .get(&key)
            .await
            .map_err(|err| {
                let err_msg = format!("Failed to fetch state @ {key}: {err:?}");
                error!(error = %err, message = err_msg);
                RpcError::Nats(err_msg)
            })
            .map(|b| b.map(|v| v.to_vec()))
    }

    #[instrument(level = "debug", skip(self))]
    pub async fn remove_state(
        &self,
        actor_role: &ActorRole,
        entity_name: &str,
        key: &str,
    ) -> Result<()> {
        let key = state_key(actor_role, entity_name, key);
        // We use a purge here instead of delete because we don't care about maintaining history (that comes from the event log)
        self.bucket
            .purge(&key)
            .await
            .map_err(|e| {
                let err_msg = format!("Failed to delete state @ {key}: {e:?}");
                error!(error = %e, message = err_msg);
                RpcError::Nats(err_msg)
            })
            .map(|_| ())
    }
}

fn state_key(role: &ActorRole, entity_name: &str, key: &str) -> String {
    match role {
        ActorRole::Aggregate => format!("agg.{entity_name}.{key}"),
        ActorRole::ProcessManager => format!("pm.{entity_name}.{key}"),
        _ => {
            error!("Attempted to get a state key for an unsupported actor role: {role:?}");
            "".to_string()
        }
    }
}

async fn get_or_create_bucket(js: &Context) -> Result<Store> {
    if let Ok(store) = js
        .get_key_value(STATE_BUCKET_NAME)
        .await
        .map_err(|e| RpcError::Nats(e.to_string()))
    {
        Ok(store)
    } else {
        Ok(js
            .create_key_value(KvConfig {
                bucket: STATE_BUCKET_NAME.to_string(),
                description: "Concordance state for aggregates and process managers".to_string(),
                history: 1,
                ..Default::default()
            })
            .await
            .map_err(|e| RpcError::Nats(e.to_string()))?)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        config::ActorRole,
        natsclient::test::{clear_streams, create_js_context},
        state::EntityState,
    };

    #[tokio::test]
    async fn test_get_bucket_returns_error() {
        // This is here because the behavior in async nats is undocumented so I want to catch
        // a regression if this changes. The assumption is we get an Err result for non-existent
        // buckets.
        let js = create_js_context().await;
        clear_streams(js.clone()).await;

        let kv = js.get_key_value("you_shall_not_pass_bucket").await;
        assert!(kv.is_err());
    }

    #[tokio::test]
    async fn state_round_trip() {
        let js = create_js_context().await;
        clear_streams(js.clone()).await;
        let state = EntityState::new_from_context(&js).await.unwrap();

        state
            .write_state(
                &ActorRole::Aggregate,
                "bankaccount",
                "ACT123",
                b"bru do you even state".to_vec(),
            )
            .await
            .unwrap();
        let data = state
            .fetch_state(&ActorRole::Aggregate, "bankaccount", "ACT123")
            .await
            .unwrap();
        assert_eq!(data, Some(b"bru do you even state".to_vec()));
    }

    #[tokio::test]
    async fn state_delete_item() {
        let js = create_js_context().await;
        clear_streams(js.clone()).await;
        let state = EntityState::new_from_context(&js).await.unwrap();
        state
            .write_state(
                &ActorRole::Aggregate,
                "bankaccount",
                "ACT123",
                b"bru do you even state".to_vec(),
            )
            .await
            .unwrap();
        state
            .remove_state(&ActorRole::Aggregate, "bankaccount", "ACT123")
            .await
            .unwrap();

        let data = state
            .fetch_state(&ActorRole::Aggregate, "bankaccount", "ACT123")
            .await
            .unwrap();
        assert!(data.is_none());

        // remove of non-existent doesn't panic
        assert!(state
            .remove_state(&ActorRole::Aggregate, "bankaccount", "ACT123")
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn query_nonexistent_state() {
        let js = create_js_context().await;
        clear_streams(js.clone()).await;

        let state = EntityState::new_from_context(&js).await.unwrap();
        let data = state
            .fetch_state(
                &ActorRole::Aggregate,
                "bankaccount",
                "never_gonna_let_you_down",
            )
            .await
            .unwrap();
        assert!(data.is_none());
    }
}
