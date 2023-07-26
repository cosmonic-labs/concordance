//! # wasmCloud Provider Implementation
//! This module contains the trait implementation mandatory for building a wasmCloud capability provider

use async_trait::async_trait;
use tracing::{error, instrument, warn};
use wasmbus_rpc::core::{HealthCheckRequest, HealthCheckResponse};
use wasmbus_rpc::provider::prelude::*;

use crate::config::{ActorRole, BaseConfiguration, InterestConstraint, InterestDeclaration};
use crate::consumers::{CommandConsumer, ConsumerManager, EventConsumer};
use crate::Result;

use crate::natsclient::NatsClient;
use crate::state::EntityState;
use crate::workers::{
    AggregateCommandWorker, AggregateEventWorker, GeneralEventWorker, ProcessManagerWorker,
};

#[derive(Clone, Provider)]
pub struct ConcordanceProvider {
    nc: async_nats::Client,
    consumer_manager: ConsumerManager,
    js: async_nats::jetstream::Context,
    state: EntityState,
}

impl ConcordanceProvider {
    pub async fn try_new(base_config: BaseConfiguration) -> Result<ConcordanceProvider> {
        let nc = base_config.get_nats_connection().await?;
        let js = if let Some(ref domain) = base_config.js_domain {
            async_nats::jetstream::with_domain(nc.clone(), domain)
        } else {
            async_nats::jetstream::new(nc.clone())
        };

        let client = NatsClient::new(js.clone());
        let (e, c) = client.ensure_streams().await.unwrap();
        let cm = ConsumerManager::new(e, c);
        let state = EntityState::new_from_context(&js).await?;

        Ok(ConcordanceProvider {
            nc,
            consumer_manager: cm,
            state,
            js,
        })
    }

    /// Adds a consumer and the appropriate worker to the provider's consumer manager, which will in turn create or
    /// bind to an existing NATS consumer
    async fn add_consumer(&self, decl: &InterestDeclaration) -> RpcResult<bool> {
        use ActorRole::*;
        use InterestConstraint::*;
        Ok(match (&decl.interest_constraint, &decl.role) {
            (Commands, _) => self.add_aggregate_cmd_consumer(decl).await,
            (Events, ProcessManager) => self.add_process_manager_consumer(decl).await,
            (Events, Projector) | (Events, Notifier) => self.add_general_event_consumer(decl).await,
            (Events, Aggregate) => self.add_aggregate_event_consumer(decl).await,
            (a, b) => {
                warn!("Unsupported combination of consumer and worker: {a:?} {b:?}. Ignoring.");
                false
            }
        })
    }

    /// Adds a consumer to the manager for notifiers. This is currently a generic worker because
    /// notifiers have just the simple stateless event handler contract
    async fn add_general_event_consumer(&self, decl: &InterestDeclaration) -> bool {
        if let Err(e) = self
            .consumer_manager
            .add_consumer::<GeneralEventWorker, EventConsumer>(
                decl.to_owned(),
                GeneralEventWorker::new(
                    self.nc.clone(),
                    self.js.clone(),
                    decl.clone(),
                    self.state.clone(),
                ),
            )
            .await
        {
            error!(
                "Failed to add event consumer for {} ({}): {}",
                decl.entity_name, decl.actor_id, e
            );
            return false;
        }
        true
    }

    async fn add_process_manager_consumer(&self, decl: &InterestDeclaration) -> bool {
        if let Err(e) = self
            .consumer_manager
            .add_consumer::<ProcessManagerWorker, EventConsumer>(
                decl.to_owned(),
                ProcessManagerWorker::new(
                    self.nc.clone(),
                    self.js.clone(),
                    decl.clone(),
                    self.state.clone(),
                ),
            )
            .await
        {
            error!(
                "Failed to add event consumer for {} ({}): {}",
                decl.entity_name, decl.actor_id, e
            );
            return false;
        }
        true
    }

    /// This is a command consumer that uses an aggregate-specific worker. This worker
    /// knows how to supply the aggregate with its internal state when applying
    /// the command. Note that aggregates can't change state during command
    /// application.
    async fn add_aggregate_cmd_consumer(&self, decl: &InterestDeclaration) -> bool {
        if let Err(e) = self
            .consumer_manager
            .add_consumer::<AggregateCommandWorker, CommandConsumer>(
                decl.to_owned(),
                AggregateCommandWorker::new(
                    self.nc.clone(),
                    self.js.clone(),
                    decl.clone(),
                    self.state.clone(),
                ),
            )
            .await
        {
            error!(
                "Failed to add command consumer for {} ({}): {}",
                decl.entity_name, decl.actor_id, e
            );
            return false;
        }
        true
    }

    /// This will add an event consumer for aggregates that will pull events, ask the aggregate
    /// to apply the event to state, and persist the state. No events or commands are emitted
    /// by an aggregate event consumer
    async fn add_aggregate_event_consumer(&self, decl: &InterestDeclaration) -> bool {
        if let Err(e) = self
            .consumer_manager
            .add_consumer::<AggregateEventWorker, EventConsumer>(
                decl.to_owned(),
                AggregateEventWorker::new(
                    self.nc.clone(),
                    self.js.clone(),
                    decl.clone(),
                    self.state.clone(),
                ),
            )
            .await
        {
            error!(
                "Failed to add event consumer for {} ({}): {}",
                decl.entity_name, decl.actor_id, e
            );
            return false;
        }
        true
    }
}

impl ProviderDispatch for ConcordanceProvider {}

#[async_trait]
impl ProviderHandler for ConcordanceProvider {
    async fn health_request(&self, _arg: &HealthCheckRequest) -> RpcResult<HealthCheckResponse> {
        Ok(HealthCheckResponse {
            healthy: true,
            message: None,
        })
    }

    #[instrument(level = "info", skip(self, ld), fields(actor_id = %ld.actor_id, provider_id = %ld.provider_id, link_name = %ld.link_name))]
    async fn put_link(&self, ld: &LinkDefinition) -> RpcResult<bool> {
        let decls = match InterestDeclaration::from_linkdefinition(ld.clone()) {
            Ok(decls) => decls,
            Err(e) => {
                error!("Failed to derive interest declarations from link definition. Aborting due to error: {e}");
                return Ok(false);
            }
        };

        for decl in &decls {
            if !self.add_consumer(decl).await? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Notify the provider that the link is dropped
    #[allow(unused_variables)]
    async fn delete_link(&self, actor_id: &str) {
        //let _ = self.router.unregister_interest(actor_id);
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::collections::HashMap;

    use async_nats::jetstream::consumer::AckPolicy;
    use wasmbus_rpc::{core::LinkDefinition, provider::ProviderHandler, wascap::prelude::KeyPair};

    use crate::{
        config::BaseConfiguration,
        natsclient::test::{clear_streams, create_js_context},
        wcprovider::ConcordanceProvider,
    };

    #[tokio::test]
    async fn test_linkdef_to_consumers() {
        let js = create_js_context().await;
        clear_streams(js.clone()).await;

        let cp = ConcordanceProvider::try_new(BaseConfiguration::default())
            .await
            .unwrap();

        cp.put_link(&make_link_definition(
            "aggregate",
            "bankaccount",
            "bankaccount",
        ))
        .await
        .unwrap();

        cp.put_link(&make_link_definition(
            "projector",
            "bankaccount",
            "account_opened,account_updated,amount_withdrawn,amount_deposited",
        ))
        .await
        .unwrap();

        cp.put_link(&make_link_definition(
            "process_manager",
            "bankaccount",
            r#"{"start": "account_opened", "advance": ["account_updated","amount_withdrawn","amount_deposited"], "stop": ["account_closed"]}"#,
        ))
        .await
        .unwrap();

        cp.put_link(&make_link_definition(
            "notifier",
            "bankaccount",
            "account_opened",
        ))
        .await
        .unwrap();

        // Verify that the provider has created the consumers corresponding to the linkdefs
        let stream = js.get_stream("CC_EVENTS").await.unwrap();
        assert_eq!(
            stream
                .consumer_info("NOTIFIER_bankaccount")
                .await
                .unwrap()
                .config
                .ack_policy,
            AckPolicy::Explicit
        );
        assert_eq!(
            stream
                .consumer_info("AGG_EVT_bankaccount")
                .await
                .unwrap()
                .config
                .ack_policy,
            AckPolicy::Explicit
        );
        assert_eq!(
            stream
                .consumer_info("PM_bankaccount")
                .await
                .unwrap()
                .config
                .ack_policy,
            AckPolicy::Explicit
        );
        assert_eq!(
            stream
                .consumer_info("PROJ_bankaccount")
                .await
                .unwrap()
                .config
                .ack_policy,
            AckPolicy::Explicit
        );

        let stream = js.get_stream("CC_COMMANDS").await.unwrap();
        let info = stream.consumer_info("AGG_CMD_bankaccount").await.unwrap();
        assert_eq!(info.config.ack_policy, AckPolicy::Explicit);
        assert_eq!(info.config.filter_subject, "cc.commands.bankaccount");

        clear_streams(js).await;
    }

    fn make_link_definition(role: &str, entity_name: &str, interest: &str) -> LinkDefinition {
        let mut ld = LinkDefinition::default();
        ld.actor_id = KeyPair::new_module().public_key();
        ld.provider_id = "VXXCONCORDANCE".to_string();
        ld.contract_id = "cosmonic:eventsourcing".to_string();
        ld.values = HashMap::from([
            ("role".to_string(), role.to_string()),
            ("name".to_string(), entity_name.to_string()),
            ("interest".to_string(), interest.to_string()),
        ]);
        ld
    }
}
