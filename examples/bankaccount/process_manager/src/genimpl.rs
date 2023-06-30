// TODO: unhardcode this opinion
use crate::eventsourcing::*;

use crate::*;

use wasmcloud_interface_logging::{debug, error};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, ProcessManagerService)]
pub(crate) struct BankaccountProcessManagerImpl {}

#[async_trait]
impl ProcessManagerService for BankaccountProcessManagerImpl {
    async fn handle_event(
        &self,
        _ctx: &Context,
        arg: &EventWithState,
    ) -> RpcResult<ProcessManagerAck> {
        let state: Option<BankaccountProcessManagerState> = arg
            .state
            .clone()
            .map(|bytes| deserialize_json(&bytes).unwrap_or_default());

        Ok(match arg.event.event_type.as_str() {
            WireTransferRequested::TYPE => {
                BankaccountProcessManager::handle_wire_transfer_requested(
                    self,
                    deserialize_json(&arg.event.payload)?,
                    state,
                )?
            }
            WireFundsReserved::TYPE => BankaccountProcessManager::handle_wire_funds_reserved(
                self,
                deserialize_json(&arg.event.payload)?,
                state,
            )?,
            InterbankTransferCompleted::TYPE => {
                BankaccountProcessManager::handle_interbank_transfer_completed(
                    self,
                    deserialize_json(&arg.event.payload)?,
                    state,
                )?
            }
            InterbankTransferFailed::TYPE => {
                BankaccountProcessManager::handle_interbank_transfer_failed(
                    self,
                    deserialize_json(&arg.event.payload)?,
                    state,
                )?
            }
            InterbankTransferInitiated::TYPE => {
                BankaccountProcessManager::handle_interbank_transfer_initiated(
                    self,
                    deserialize_json(&arg.event.payload)?,
                    state,
                )?
            }
            e => {
                debug!("Unexpected event received '{e}'. Acking and moving on - Is interest configured properly??");
                ProcessManagerAck::ok(state, vec![])
            }
        })
    }
}

fn deserialize_json<'de, T: Deserialize<'de>>(buf: &'de [u8]) -> RpcResult<T> {
    serde_json::from_slice(buf).map_err(|e| format!("Deserialization failure: {e:?}").into())
}

fn serialize_json<T: Serialize>(data: &T) -> RpcResult<Vec<u8>> {
    serde_json::to_vec(data).map_err(|e| format!("Serialization failure: {e:?}").into())
}

impl ProcessManagerAck {
    pub fn ok(state: Option<impl Serialize>, cmds: CommandList) -> Self {
        Self {
            state: state.map(|s| serialize_json(&s).unwrap_or_default()),
            commands: cmds,
        }
    }
}
