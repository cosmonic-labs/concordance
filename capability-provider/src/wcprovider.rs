//! # wasmCloud Provider Implementation
//! This module contains the trait implementation mandatory for building a wasmCloud capability provider

use async_trait::async_trait;
use tracing::{debug, error, info, instrument, trace, warn};
use wasmbus_rpc::core::{HealthCheckRequest, HealthCheckResponse};
use wasmbus_rpc::provider::prelude::*;

use crate::config::{ActorInterest, BaseConfiguration, InterestConstraint, InterestDeclaration};
use crate::consumers::{
    CommandConsumer, CommandWorker, ConsumerManager, EventConsumer, EventWorker,
};
use crate::router::Router;
use crate::state::EntityState;

#[derive(Clone, Provider)]
pub struct ConcordanceProvider {
    base_config: BaseConfiguration,
    consumer_manager: ConsumerManager,
    js: async_nats::jetstream::Context,
    state: EntityState,
    router: Router,
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
        let Ok(decls) = InterestDeclaration::from_linkdefinition(ld) else {
            error!("Failed to derive interest declarations from link definition. Aborting.");
            return Ok(false)
        };
        for decl in &decls {
            if decl.interest_constraint == InterestConstraint::Commands {
                if let Err(e) = self
                    .consumer_manager
                    .add_consumer::<CommandWorker, CommandConsumer>(
                        decl.to_owned(),
                        CommandWorker::new(self.js.clone(), decl.clone(), self.state.clone()),
                    )
                    .await
                {
                    error!(
                        "Failed to add command consumer for {} ({}): {}",
                        decl.entity_name, decl.actor_id, e
                    );
                    return Ok(false);
                }
            } else {
                if let Err(e) = self
                    .consumer_manager
                    .add_consumer::<EventWorker, EventConsumer>(
                        decl.to_owned(),
                        EventWorker::new(self.js.clone(), decl.clone(), self.state.clone()),
                    )
                    .await
                {
                    error!(
                        "Failed to add event consumer for {} ({}): {}",
                        decl.entity_name, decl.actor_id, e
                    );
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    /// Notify the provider that the link is dropped
    #[allow(unused_variables)]
    async fn delete_link(&self, actor_id: &str) {
        let _ = self.router.unregister_interest(actor_id);
    }
}
