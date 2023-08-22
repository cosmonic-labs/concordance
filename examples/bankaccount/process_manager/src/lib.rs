use bankaccount_model::commands::*;
use bankaccount_model::events::*;
use bankaccount_model::state::*;

use serde::{Deserialize, Serialize};

const STREAM: &str = "bankaccount";

concordance_gen::generate!({
    path: "../bankaccount-model.ttl",
    role: "process_manager",
    entity: "bankaccount"
});

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
            vec![OutputCommand::new(
                ReserveFunds::TYPE,
                &cmd,
                STREAM,
                &cmd.account_number,
            )],
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
            vec![OutputCommand::new(
                InitiateInterbankTransfer::TYPE,
                &cmd,
                STREAM,
                &cmd.account_number,
            )],
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
        Ok(ProcessManagerAck::ok(
            None::<BankaccountProcessManagerState>,
            vec![OutputCommand::new(
                WithdrawReservedFunds::TYPE,
                &cmd,
                STREAM,
                &state.account_number,
            )],
        ))
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
