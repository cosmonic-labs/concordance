//! # wasmCloud Provider Implementation
//! This module contains the trait implementation mandatory for building a wasmCloud capability provider

use async_trait::async_trait;
use tracing::{debug, error, info, instrument, trace, warn};
use wasmbus_rpc::core::{HealthCheckRequest, HealthCheckResponse};
use wasmbus_rpc::provider::prelude::*;

use crate::config::{ActorInterest, BaseConfiguration, InterestDeclaration};
use crate::router::Router;

#[derive(Clone, Provider)]
pub struct ConcordanceProvider {
    base_config: BaseConfiguration,
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
        let Ok(decl)  = InterestDeclaration::try_from(ld) else {
            return Ok(false)
        };
        self.router.register_interest(decl);
        // self.nats.ensure_consumer_subscription(decl);
        // self.nats.ensure_consumer_subscriptions(self.router.get_consumer_list());
        todo!()
        //Ok(self.router.apply_linkdef(ld))
    }

    /// Notify the provider that the link is dropped
    #[allow(unused_variables)]
    async fn delete_link(&self, actor_id: &str) {
        let _ = self.router.unregister_interest(actor_id);
    }
}
