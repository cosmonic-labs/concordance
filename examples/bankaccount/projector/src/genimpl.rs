// CODEGEN notice this is an automatically generated file. Do not modify this directly.

use crate::*;

use wasmcloud_interface_logging::error;

// One-way event handler implementation for .Bankaccount


#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, StatelessEventHandlerService)]
pub(crate) struct BankaccountProjectorImpl {}

#[async_trait]
impl StatelessEventHandlerService for BankaccountProjectorImpl {    
    async fn apply_stateless_event(&self, _ctx: &Context, arg: &Event) -> RpcResult<StatelessAck> {        

        Ok(match arg.event_type.as_str() {
            AccountCreated::TYPE => {
                BankaccountProjector::handle_account_created(
                    self,
                    deserialize_json(&arg.payload)?,                    
                ).into() },
            WireFundsReserved::TYPE => {
                BankaccountProjector::handle_wire_funds_reserved(
                    self,
                    deserialize_json(&arg.payload)?,                    
                ).into() },
            FundsWithdrawn::TYPE => {
                BankaccountProjector::handle_funds_withdrawn(
                    self,
                    deserialize_json(&arg.payload)?,                    
                ).into() },
            WireFundsReleased::TYPE => {
                BankaccountProjector::handle_wire_funds_released(
                    self,
                    deserialize_json(&arg.payload)?,                    
                ).into() },
            InterbankTransferInitiated::TYPE => {
                BankaccountProjector::handle_interbank_transfer_initiated(
                    self,
                    deserialize_json(&arg.payload)?,                    
                ).into() },
            WireTransferRequested::TYPE => {
                BankaccountProjector::handle_wire_transfer_requested(
                    self,
                    deserialize_json(&arg.payload)?,                    
                ).into() },
            FundsDeposited::TYPE => {
                BankaccountProjector::handle_funds_deposited(
                    self,
                    deserialize_json(&arg.payload)?,                    
                ).into() },
            ReservedFundsWithdrawn::TYPE => {
                BankaccountProjector::handle_reserved_funds_withdrawn(
                    self,
                    deserialize_json(&arg.payload)?,                    
                ).into() },
            e =>   {
                debug!("Unexpected event received '{e}'. Acking and moving on.");
                StatelessAck::ok()
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

impl Into<StatelessAck> for Result<(), RpcError> {
    fn into(self) -> StatelessAck {
        match self {
            Ok(_) => StatelessAck::ok(),
            Err(e) => {
                error!("Error handling event: {:?}", e);
                StatelessAck::error(e.to_string())
            }
        }
    }
}

impl StatelessAck {
    pub fn ok() -> Self {
        Self { error: None, succeeded: true }
    }

    pub fn error(msg: String) -> Self {
        Self {
            error: Some(msg),
            succeeded: false
        }
    }
}