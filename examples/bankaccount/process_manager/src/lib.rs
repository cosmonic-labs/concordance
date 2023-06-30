use eventsourcing::*;
use genimpl::BankaccountProcessManagerImpl;
use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;

use bankaccount_model::commands::*;
use bankaccount_model::events::*;
use bankaccount_model::state::*;
use wasmcloud_interface_logging::{error, info};

#[allow(dead_code)]
mod eventsourcing;

mod genimpl;
mod system_traits;

use system_traits::*;

const STREAM: &str = "bankaccount";

impl BankaccountProcessManager for BankaccountProcessManagerImpl {
    /// Initiates a new process for managing wire transfers
    fn handle_wire_transfer_requested(
        &self,
        input: WireTransferRequested,
        _state: Option<BankaccountProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        let new_state = BankaccountProcessManagerState::new(&input);

        let cmd = ReserveFunds {
            account_number: input.account_number,
            amount: input.amount,
            wire_transfer_id: input.wire_transfer_id.to_string(),
        };

        Ok(ProcessManagerAck::ok(
            Some(new_state),
            vec![OutputCommand {
                command_type: ReserveFunds::TYPE.to_string(),
                json_payload: serde_json::to_vec(&cmd).unwrap_or_default(),
                aggregate_stream: STREAM.to_string(),
                aggregate_key: cmd.account_number.to_string(), // all commands are targeted at instances of an aggregate
            }],
        ))
    }

    fn handle_wire_funds_reserved(
        &self,
        event: WireFundsReserved,
        state: Option<BankaccountProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        let state = state.unwrap_or_default();
        let state = BankaccountProcessManagerState {
            status: TransferStatus::FundsReserved,
            ..state
        };

        let cmd = InitiateInterbankTransfer {
            account_number: event.account_number,
            amount: event.amount,
            wire_transfer_id: event.wire_transfer_id,
            expiration_in_days: 3, // this doesn't do anything, it's just an example of augmenting domain-specific data on a cmd
            target_account_number: state.target_account_number.to_string(),
            target_routing_number: state.target_routing_number.to_string(),
        };

        Ok(ProcessManagerAck::ok(
            Some(state),
            vec![OutputCommand {
                command_type: InitiateInterbankTransfer::TYPE.to_string(),
                json_payload: serde_json::to_vec(&cmd).unwrap_or_default(),
                aggregate_stream: STREAM.to_string(),
                aggregate_key: cmd.account_number.to_string(),
            }],
        ))
    }

    fn handle_interbank_transfer_completed(
        &self,
        input: InterbankTransferCompleted,
        state: Option<BankaccountProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        let state = state.unwrap_or_default();

        let cmd = WithdrawReservedFunds {
            account_number: state.account_number.to_string(),
            wire_transfer_id: input.wire_transfer_id.to_string(),
            amount: state.amount,
        };

        // Returning `None` for the state here guarantees this process state is deleted
        Ok(
            ProcessManagerAck::ok(
                None::<BankaccountProcessManagerState>,
                vec![
                    OutputCommand {
                        command_type: WithdrawReservedFunds::TYPE.to_string(),
                        json_payload: serde_json::to_vec(&cmd).unwrap_or_default(),
                        aggregate_stream: STREAM.to_string(),
                        aggregate_key: state.account_number.to_string(),
                    }
                ]
            )
        )    
    }

    fn handle_interbank_transfer_failed(
        &self,
        input: InterbankTransferFailed,
        state: Option<BankaccountProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        todo!()
    }

    fn handle_interbank_transfer_initiated(
        &self,
        input: InterbankTransferInitiated,
        state: Option<BankaccountProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        todo!()
    }
}

/*
#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, ProcessManagerService)]
struct InterbankTransferProcessManager {}

#[async_trait]
impl ProcessManagerService for InterbankTransferProcessManager {
    async fn handle_event(
        &self,
        _ctx: &Context,
        arg: &EventWithState,
    ) -> RpcResult<ProcessManagerAck> {
        info!(
            "Process manager handling event: {}",
            arg.event.event_type.as_str()
        );

        let state: Option<BankaccountProcessManagerState> = arg
            .state
            .clone()
            .map(|bytes| deserialize(&bytes).unwrap_or_default());

        match arg.event.event_type.as_str() {
            WIRE_TRANSFER_REQUESTED_EVENT_TYPE => {
                handle_wire_transfer_requested(
                    deserialize(&arg.event.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
                )
                .await
            }
            WIRE_FUNDS_RESERVED_EVENT_TYPE => handle_wire_funds_reserved(
                state,
                deserialize(&arg.event.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            INTERBANK_TRANSFER_INITIATED_EVENT_TYPE => handle_interbank_xfer_initiated(
                state,
                deserialize(&arg.event.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            INTERBANK_TRANSFER_COMPLETED_EVENT_TYPE => handle_interbank_xfer_completed(
                state,
                deserialize(&arg.event.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            INTERBANK_TRANSFER_FAILED_EVENT_TYPE => handle_interbank_xfer_failed(
                state,
                deserialize(&arg.event.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            e => {
                error!("Unsupported event type: {e}. Interest configuration for this process manager is probably incorect.");
                Ok(ProcessManagerAck {
                    state: arg.state.clone(),
                    commands: vec![],
                })
            }
        }
    }
}

/// Assigns a new transfer transaction ID and issues a command requesting the funds be reserved on the account
async fn handle_wire_transfer_requested(
    event: WireTransferRequested,
) -> RpcResult<ProcessManagerAck> {
    let new_state = BankaccountProcessManagerState::new(&event).await;

    let cmd = ReserveFunds {
        account_number: event.account_number,
        amount: event.amount,
        wire_transfer_id: new_state.wire_transfer_id.to_string(),
    };

    Ok(ProcessManagerAck {
        commands: vec![OutputCommand {
            command_type: RESERVE_FUNDS_TYPE.to_string(),
            json_payload: serde_json::to_vec(&cmd).unwrap_or_default(),
            aggregate_stream: BANKACCOUNT_STREAM.to_string(),
            aggregate_key: cmd.account_number.to_string(), // all commands are targeted at instances of an aggregate
        }],
        state: Some(new_state.to_bytes()),
    })
}

/// In response to a notification that funds have been reserved, issue a new command to initiate the bank-to-bank transfer
fn handle_wire_funds_reserved(
    state: Option<BankaccountProcessManagerState>,
    event: WireFundsReserved,
) -> RpcResult<ProcessManagerAck> {
    let state = state.unwrap_or_default();
    let state = BankaccountProcessManagerState {
        status: TransferStatus::FundsReserved,
        ..state
    };

    let cmd = InitiateInterbankTransfer {
        account_number: event.account_number,
        amount: event.amount,
        wire_transfer_id: event.wire_transfer_id,
        expiration_in_days: 3, // this doesn't do anything, it's just an example of augmenting domain-specific data on a cmd
        target_account_number: state.target_account_number.to_string(),
        target_routing_number: state.target_routing_number.to_string(),
    };

    Ok(ProcessManagerAck {
        commands: vec![OutputCommand {
            command_type: INITIATE_TRANSFER_TYPE.to_string(),
            json_payload: serde_json::to_vec(&cmd).unwrap_or_default(),
            aggregate_stream: BANKACCOUNT_STREAM.to_string(),
            aggregate_key: cmd.account_number.to_string(),
        }],
        state: Some(state.to_bytes()),
    })
}

/// In response to this event we just update the internal state. We don't issue commands in response to this as the
/// interbank gateway will be following up with a success or fail event
fn handle_interbank_xfer_initiated(
    state: Option<BankaccountProcessManagerState>,
    _event: InterbankTransferInitiated,
) -> RpcResult<ProcessManagerAck> {
    let state = state.unwrap_or_default();
    let state = BankaccountProcessManagerState {
        status: TransferStatus::TransferInitiated,
        ..state
    };

    Ok(ProcessManagerAck {
        commands: vec![],
        state: Some(state.to_bytes()),
    })
}

/// This handles one of the two events that come back from the "interbank gateway", a stand-in for an
/// integration process or subsystem common to many banking solutions. Closes the process and issues a
/// command to withdraw the previously reserved funds.
/// NOTE: we could make this a bit more complicated by waiting for a confirmation of funds withdrawal
/// before closing the process, but we want to keep this domain as simple as possible
fn handle_interbank_xfer_completed(
    state: Option<BankaccountProcessManagerState>,
    _event: InterbankTransferCompleted,
) -> RpcResult<ProcessManagerAck> {
    let state = state.unwrap_or_default();

    let cmd = WithdrawReservedFunds {
        account_number: state.account_number.to_string(),
        wire_transfer_id: state.wire_transfer_id.to_string(),
        amount: state.amount,
    };

    // Returning `None` for the state here guarantees this process state is deleted
    Ok(ProcessManagerAck {
        commands: vec![OutputCommand {
            command_type: WITHDRAW_RESERVED_FUNDS_TYPE.to_string(),
            json_payload: serde_json::to_vec(&cmd).unwrap_or_default(),
            aggregate_stream: BANKACCOUNT_STREAM.to_string(),
            aggregate_key: state.account_number, // this is going "to" the aggregate stream, so it's the aggregate key that matters, not the PM
        }],
        state: None,
    })
}

/// In response to an indication of failure from the interbank gateway, this process manager will terminate
/// the transfer process and emit a command to release previously reserved funds. If we want a consumer-friendly
/// query of the status/results of a given process we can use a projector
fn handle_interbank_xfer_failed(
    state: Option<BankaccountProcessManagerState>,
    _event: InterbankTransferFailed,
) -> RpcResult<ProcessManagerAck> {
    let state = state.unwrap_or_default();

    let cmd = ReleaseReservedFunds {
        account_number: state.account_number.to_string(),
        wire_transfer_id: state.wire_transfer_id.to_string(),
        amount: state.amount,
    };

    // Returning `None` for the state here guarantees this process state is deleted
    Ok(ProcessManagerAck {
        commands: vec![OutputCommand {
            command_type: RELEASE_RESERVED_FUNDS_TYPE.to_string(),
            json_payload: serde_json::to_vec(&cmd).unwrap_or_default(),
            aggregate_stream: BANKACCOUNT_STREAM.to_string(),
            aggregate_key: state.account_number, // this command goes out to the aggregate bank account, so the key is the aggregate's key
        }],
        state: None,
    })
}


*/
