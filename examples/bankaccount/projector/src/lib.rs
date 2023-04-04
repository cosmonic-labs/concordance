use eventsourcing::*;
use wasmbus_rpc::actor::prelude::*;

use bankaccount_model::deserialize;
use bankaccount_model::events::*;
use wasmcloud_interface_logging::{debug, info};

#[allow(dead_code)]
mod eventsourcing;

mod store;

use eventsourcing::Event as ConcordanceEvent;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, StatelessEventHandlerService)]
struct BankAccountProjector {}

// In phase 2, code generation will produce strongly-typed handler traits that will
// manage the match statement internally prior to developer functions.
//
// pub trait BankAccountProjectorService: StatelessEventHandlerService {
//     async fn apply_account_created(arg: &AccountCreatedEvent) -> RpcResult<StatelessAck>;
// }

#[async_trait]
impl StatelessEventHandlerService for BankAccountProjector {
    async fn apply_stateless_event(
        &self,
        ctx: &Context,
        arg: &ConcordanceEvent,
    ) -> RpcResult<StatelessAck> {
        info!("Bank account projector handling event {}", arg.event_type);

        let res = match arg.event_type.as_str() {
            ACCOUNT_CREATED_TYPE => {
                store::initialize_account(
                    ctx,
                    deserialize(&arg.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
                )
                .await
            }
            FUNDS_DEPOSITED_EVENT_TYPE => {
                store::record_deposit(
                    ctx,
                    deserialize(&arg.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
                )
                .await
            }
            FUNDS_WITHDRAWN_EVENT_TYPE => {
                store::record_withdrawal(
                    ctx,
                    deserialize(&arg.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
                )
                .await
            }
            WIRE_FUNDS_RESERVED_EVENT_TYPE => {
                store::record_funds_reserved(
                    ctx,
                    deserialize(&arg.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
                )
                .await
            }
            WIRE_FUNDS_RELEASED_EVENT_TYPE => {
                store::release_reserved_funds(
                    ctx,
                    deserialize(&arg.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
                )
                .await
            }
            _ => {
                debug!(
                    "Bank account projector acking unwanted event {} and moving on",
                    arg.event_type
                );
                Ok(())
            }
        };
        if let Err(e) = res {
            Ok(StatelessAck {
                error: Some(e.to_string()),
                succeeded: false,
            })
        } else {
            Ok(StatelessAck {
                error: None,
                succeeded: true,
            })
        }
    }
}
