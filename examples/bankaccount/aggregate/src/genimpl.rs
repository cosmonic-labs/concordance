// TODO: unhardcode this
use crate::eventsourcing::*;

use crate::*;

use wasmcloud_interface_logging::error;


#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, AggregateService)]
pub(crate) struct BankaccountAggregateImpl {}

#[async_trait]
impl AggregateService for BankaccountAggregateImpl {
    async fn handle_command(&self, _ctx: &Context, arg: &StatefulCommand) -> RpcResult<EventList> {
          let state: Option<BankaccountAggregateState> = arg
            .state
            .clone()
            .map(|bytes| deserialize_json(&bytes).unwrap_or_default());

        match arg.command_type.as_str() {
                CreateAccount::TYPE => {
                    BankaccountAggregate::handle_create_account(
                        self,
                        deserialize_json(&arg.payload)?,                        
                        state
                    )                   
                },
                
                ReserveFunds::TYPE => {
                    BankaccountAggregate::handle_reserve_funds(
                        self,
                        deserialize_json(&arg.payload)?,                        
                        state
                    )                   
                },
                
                ReleaseReservedFunds::TYPE => {
                    BankaccountAggregate::handle_release_reserved_funds(
                        self,
                        deserialize_json(&arg.payload)?,                        
                        state
                    )                   
                },
                
                WithdrawFunds::TYPE => {
                    BankaccountAggregate::handle_withdraw_funds(
                        self,
                        deserialize_json(&arg.payload)?,                        
                        state
                    )                   
                },
                
                DepositFunds::TYPE => {
                    BankaccountAggregate::handle_deposit_funds(
                        self,
                        deserialize_json(&arg.payload)?,                        
                        state
                    )                   
                },
                
                RequestWireTransfer::TYPE => {
                    BankaccountAggregate::handle_request_wire_transfer(
                        self,
                        deserialize_json(&arg.payload)?,                        
                        state
                    )                   
                },
                
                InitiateInterbankTransfer::TYPE => {
                    BankaccountAggregate::handle_initiate_interbank_transfer(
                        self,
                        deserialize_json(&arg.payload)?,                        
                        state
                    )                   
                },
                
                WithdrawReservedFunds::TYPE => {
                    BankaccountAggregate::handle_withdraw_reserved_funds(
                        self,
                        deserialize_json(&arg.payload)?,                        
                        state
                    )                   
                },
                
            e => {
                error!("Unsupported command type: {e}. Interest configuration for this Aggregate is probably incorect.");
                Ok(vec![])
            }
        }
    }

    async fn apply_event(&self, _ctx: &Context, arg: &EventWithState) -> RpcResult<StateAck> {
        let state: Option<BankaccountAggregateState> = arg
         .state
         .clone()
         .map(|bytes| deserialize_json(&bytes).unwrap_or_default());

        Ok(match arg.event.event_type.as_str() {
            AccountCreated::TYPE => {
                BankaccountAggregate::apply_account_created(
                    self,
                    deserialize_json(&arg.event.payload)?,                    
                    state)?
                },
            WireFundsReserved::TYPE => {
                BankaccountAggregate::apply_wire_funds_reserved(
                    self,
                    deserialize_json(&arg.event.payload)?,                    
                    state)?
                },
            FundsWithdrawn::TYPE => {
                BankaccountAggregate::apply_funds_withdrawn(
                    self,
                    deserialize_json(&arg.event.payload)?,                    
                    state)?
                },
            WireFundsReleased::TYPE => {
                BankaccountAggregate::apply_wire_funds_released(
                    self,
                    deserialize_json(&arg.event.payload)?,                    
                    state)?
                },
            InterbankTransferInitiated::TYPE => {
                BankaccountAggregate::apply_interbank_transfer_initiated(
                    self,
                    deserialize_json(&arg.event.payload)?,                    
                    state)?
                },
            WireTransferRequested::TYPE => {
                BankaccountAggregate::apply_wire_transfer_requested(
                    self,
                    deserialize_json(&arg.event.payload)?,                    
                    state)?
                },
            FundsDeposited::TYPE => {
                BankaccountAggregate::apply_funds_deposited(
                    self,
                    deserialize_json(&arg.event.payload)?,                    
                    state)?
                },
            ReservedFundsWithdrawn::TYPE => {
                BankaccountAggregate::apply_reserved_funds_withdrawn(
                    self,
                    deserialize_json(&arg.event.payload)?,                    
                    state)?
                },
            e =>   {
                debug!("Non-state-mutating event received '{e}'. Acking and moving on.");
                StateAck::ok(state)
            }
        })        
    }
}

fn deserialize_json<'de, T: Deserialize<'de>>(
    buf: &'de [u8],
) -> RpcResult<T> {
    serde_json::from_slice(buf).map_err(|e| format!("Deserialization failure: {e:?}").into())
}

fn serialize_json<T: Serialize>(data: &T) -> RpcResult<Vec<u8>> {
    serde_json::to_vec(data).map_err(|e| format!("Serialization failure: {e:?}").into())
}
