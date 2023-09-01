use serde::{Deserialize, Serialize};

concordance_gen::generate!({
    path: "../eventcatalog",
    role: "process_manager",
    entity: "wire transfer"
});

#[async_trait]
impl WireTransferProcessManager for WireTransferProcessManagerImpl {
    async fn handle_funds_released(
        &self,
        _input: FundsReleased,
        _state: Option<WireTransferProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        // release of funds is the termination of a transfer process
        Ok(ProcessManagerAck::ok(
            None::<WireTransferProcessManagerState>,
            vec![],
        ))
    }
    
    async fn handle_funds_committed(
        &self,
        _input: FundsCommitted,
        _state: Option<WireTransferProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        // commitment of funds is the termination of a transfer process
        Ok(ProcessManagerAck::ok(
            None::<WireTransferProcessManagerState>,
            vec![],
        ))
    }

    async fn handle_funds_reserved(
        &self,
        _input: FundsReserved,
        state: Option<WireTransferProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        let Some(mut state) = state else {
            return Ok(ProcessManagerAck::ok(
                None::<WireTransferProcessManagerState>,
                vec![],
            ));
        };
        state.status = TransferStatus::FundsReserved;
        Ok(ProcessManagerAck::ok(Some(state), vec![]))
    }

    async fn handle_wire_transfer_succeeded(
        &self,
        input: WireTransferSucceeded,
        state: Option<WireTransferProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        let Some(mut state) = state else {
            return Ok(ProcessManagerAck::ok(
                None::<WireTransferProcessManagerState>,
                vec![],
            ));
        };
        state.status = TransferStatus::TransferCompleted;
        let cmd = CommitFunds {
            account_number: state.account_number.to_string(),
            customer_id: state.customer_id.to_string(),
            wire_transfer_id: input.wire_transfer_id.to_string(),
        };

        Ok(ProcessManagerAck::ok(
            Some(state),
            vec![OutputCommand::new(
                CommitFunds::TYPE,
                &cmd,
                STREAM,
                &cmd.account_number,
            )],
        ))
    }

    async fn handle_wire_transfer_initiated(
        &self,
        input: WireTransferInitiated,
        _state: Option<WireTransferProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        let state = WireTransferProcessManagerState::new(&input);

        let cmd = ReserveFunds {
            customer_id: input.customer_id,
            account_number: input.account_number,
            amount: input.amount,
            wire_transfer_id: input.wire_transfer_id.to_string(),
        };

        Ok(ProcessManagerAck::ok(
            Some(state),
            vec![OutputCommand::new(
                ReserveFunds::TYPE,
                &cmd,
                STREAM,
                &cmd.account_number,
            )],
        ))
    }

    async fn handle_wire_transfer_failed(
        &self,
        input: WireTransferFailed,
        state: Option<WireTransferProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        let Some(state) = state else {
            return Ok(ProcessManagerAck::ok(
                None::<WireTransferProcessManagerState>,
                vec![],
            ));
        };
        let cmd = ReleaseFunds {
            account_number: state.account_number.to_string(),
            customer_id: state.customer_id.to_string(),
            wire_transfer_id: input.wire_transfer_id.to_string(),
        };
        Ok(ProcessManagerAck::ok(
            Some(state),
            vec![OutputCommand::new(
                ReleaseFunds::TYPE,
                &cmd,
                STREAM,
                &cmd.account_number,
            )],
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WireTransferProcessManagerState {
    pub wire_transfer_id: String,
    pub account_number: String,
    pub customer_id: String,
    pub amount: u32,
    pub target_routing_number: String,
    pub target_account_number: String,
    pub status: TransferStatus,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub enum TransferStatus {
    Requested,
    FundsReserved,
    TransferInitiated,
    TransferCompleted,
    TransferFailed,
    #[default]
    Unknown,
}

impl WireTransferProcessManagerState {
    pub fn to_bytes(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap_or_default()
    }
}

impl WireTransferProcessManagerState {
    pub fn new(event: &WireTransferInitiated) -> WireTransferProcessManagerState {
        let event = event.clone();
        WireTransferProcessManagerState {
            wire_transfer_id: event.wire_transfer_id,
            account_number: event.account_number,
            customer_id: event.customer_id,
            amount: event.amount as u32,
            target_routing_number: event.target_routing_number,
            target_account_number: event.target_account_number,
            status: TransferStatus::Requested,
        }
    }
}

const STREAM: &str = "bankaccount";
