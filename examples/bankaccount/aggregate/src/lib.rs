use eventsourcing::*;
use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;

use bankaccount_model::commands::*;
use bankaccount_model::deserialize;
use bankaccount_model::events::*;
use wasmcloud_interface_logging::debug;
use wasmcloud_interface_logging::{error, info};

#[allow(dead_code)]
mod eventsourcing;

const STREAM: &str = "bankaccount";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, AggregateService)]
struct BankAccountAggregate {}

#[async_trait]
impl AggregateService for BankAccountAggregate {
    async fn handle_command(&self, _ctx: &Context, arg: &StatefulCommand) -> RpcResult<EventList> {
        info!(
            "Bank account aggregate handling command: {}",
            arg.command_type.as_str()
        );

        let state: Option<AggregateState> = arg
            .state
            .clone()
            .map(|bytes| deserialize(&bytes).unwrap_or_default());

        Ok(match arg.command_type.as_str() {
            CREATE_ACCOUNT_TYPE => create_account(
                deserialize(&arg.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            RESERVE_FUNDS_TYPE => reserve_funds(
                state,
                deserialize(&arg.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            RELEASE_RESERVED_FUNDS_TYPE => release_reserved_funds(
                state,
                deserialize(&arg.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            DEPOSIT_FUNDS_TYPE => deposit_funds(
                deserialize(&arg.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            WIRE_TRANSFER_REQUEST_TYPE => request_wire_transfer(
                state,
                deserialize(&arg.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            INITIATE_TRANSFER_TYPE => initiate_interbank_transfer(
                state,
                deserialize(&arg.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            WITHDRAW_FUNDS_TYPE => withdraw_funds(
                state,
                deserialize(&arg.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            e => {
                error!("Unsupported command type: {e}. Interest configuration for this aggregate is probably incorect.");
                vec![]
            }
        })
    }

    async fn apply_event(&self, _ctx: &Context, arg: &EventWithState) -> RpcResult<StateAck> {
        info!(
            "Bank account aggregate handling event {}",
            arg.event.event_type
        );

        let state: Option<AggregateState> = arg
            .state
            .clone()
            .map(|bytes| deserialize(&bytes).unwrap_or_default());

        Ok(match arg.event.event_type.as_str() {
            ACCOUNT_CREATED_TYPE => apply_account_created(
                deserialize(&arg.event.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            FUNDS_DEPOSITED_EVENT_TYPE => apply_funds_deposited(
                state,
                deserialize(&arg.event.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            FUNDS_WITHDRAWN_EVENT_TYPE => apply_funds_withdrawn(
                state,
                deserialize(&arg.event.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            WIRE_FUNDS_RESERVED_EVENT_TYPE => apply_funds_reserved(
                state,
                deserialize(&arg.event.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            e => {
                debug!("Non-state-mutating event received '{e}'. Acking and moving on.");
                StateAck::ok(state)
            }
        })
    }
}

// This function doesn't use/care about pre-existing state. This creates it new
fn apply_account_created(event: AccountCreatedEvent) -> StateAck {
    let state = AggregateState {
        balance: event.initial_balance,
        min_balance: event.min_balance,
        account_number: event.account_number,
        customer_id: event.customer_id,
        reserved_amount: 0,
    };
    StateAck::ok(Some(state))
}

fn apply_funds_deposited(state: Option<AggregateState>, event: FundsDepositedEvent) -> StateAck {
    let Some(old_state) = state else {
        error!(
            "Rejecting funds deposited event. Account {} does not exist.",
            event.account_number
        );
        return StateAck::error("Account does not exist", None);
    };
    let state = AggregateState {
        balance: old_state.balance + event.amount,
        ..old_state
    };
    StateAck::ok(Some(state))
}

fn apply_funds_withdrawn(state: Option<AggregateState>, event: FundsWithdrawnEvent) -> StateAck {
    let Some(old_state) = state else {
        error!(
            "Rejecting funds withdrawn event. Account {} does not exist.",
            event.account_number
        );
        return StateAck::error("Account does not exist", None);        
    };

    let state = AggregateState {
        balance: old_state.balance - event.amount,
        ..old_state
    };
    StateAck::ok(Some(state))
}

fn apply_funds_reserved(state: Option<AggregateState>, event: WireFundsReserved) -> StateAck {
    let Some(old_state) = state else {
        error!(
            "Rejecting funds reserved event. Account {} does not exist.",
            event.account_number
        );
        return StateAck::error("Account does not exist", None);
    };

    let state = AggregateState {
        reserved_amount: old_state.reserved_amount + event.amount,
        ..old_state
    };
    StateAck::ok(Some(state))
}

/* --- Commands ---  */

fn create_account(cmd: CreateAccountCommand) -> EventList {
    vec![Event::new(
        ACCOUNT_CREATED_TYPE,
        STREAM,
        &AccountCreatedEvent {
            initial_balance: cmd.initial_balance,
            account_number: cmd.account_number.to_string(),
            min_balance: cmd.min_balance,
            customer_id: cmd.customer_id,
        },
    )]
}

fn request_wire_transfer(state: Option<AggregateState>, cmd: RequestWireTransfer) -> EventList {
    let Some(old_state) = state else {
        error!(
            "Rejected incoming command to request a wire transfer. Account {} does not exist.",
            cmd.account_number
        );
        return vec![];
    };

    vec![Event::new(
        WIRE_TRANSFER_REQUESTED_EVENT_TYPE,
        STREAM,
        WireTransferRequested {
            account_number: cmd.account_number.to_string(),
            wire_transfer_id: cmd.wire_transfer_id,
            amount: cmd.amount,
            customer_id: old_state.customer_id,
            target_account_number: cmd.target_account_number,
            target_routing_number: cmd.target_routing_number,
        },
    )]
}

fn initiate_interbank_transfer(
    state: Option<AggregateState>,
    cmd: InitiateInterbankTransfer,
) -> EventList {
    if state.is_none() {
        error!(
            "Rejected incoming command to initiate bank transfer. Account {} does not exist.",
            cmd.account_number
        );
        return vec![];
    };
    // NOTE: validation would occur here in a real app
    vec![Event::new(
        INTERBANK_TRANSFER_INITIATED_EVENT_TYPE,
        STREAM,
        InterbankTransferInitiated {
            account_number: cmd.account_number.to_string(),
            wire_transfer_id: cmd.wire_transfer_id,
            target_account_number: cmd.target_account_number,
            target_routing_number: cmd.target_routing_number,
        },
    )]
}

fn release_reserved_funds(state: Option<AggregateState>, cmd: ReleaseReservedFunds) -> EventList {
    let Some(old_state) = state else {
        error!(
            "Rejected incoming command to release reserved funds. Account {} does not exist.",
            cmd.account_number
        );
        return vec![];
    };
    let adj_amount = cmd.amount.min(old_state.balance);
    vec![Event::new(
        WIRE_FUNDS_RELEASED_EVENT_TYPE,
        STREAM,
        WireFundsReleased {
            account_number: cmd.account_number.to_string(),
            wire_transfer_id: cmd.wire_transfer_id.to_string(),
            amount: adj_amount,
        },
    )]
}

fn reserve_funds(state: Option<AggregateState>, cmd: ReserveFunds) -> EventList {
    let Some(old_state) = state else {
        error!(
            "Rejected incoming command to reserve funds. Account {} does not exist.",
            cmd.account_number
        );
        return vec![];
    };
    let avail_balance = old_state.balance - old_state.reserved_amount;
    if cmd.amount > avail_balance {
        // In a real-world system this might emit a failure event rather than just silently absorb the command
        error!(
            "Rejecting command to reserve funds, account {} does not have sufficient funds.",
            &cmd.account_number
        );
        vec![]
    } else {
        vec![Event::new(
            WIRE_FUNDS_RESERVED_EVENT_TYPE,
            STREAM,
            WireFundsReserved {
                account_number: cmd.account_number.to_string(),
                wire_transfer_id: cmd.wire_transfer_id,
                customer_id: old_state.customer_id.to_string(),
                amount: cmd.amount,
            },
        )]
    }
}

fn withdraw_funds(state: Option<AggregateState>, cmd: WithdrawFundsCommand) -> EventList {
    let Some(old_state) = state else {
        error!(
            "Rejected incoming command to withdraw funds. Account {} does not exist.",
            cmd.account_number
        );
        return vec![];
    };

    let avail_balance = old_state.balance - old_state.reserved_amount;
    if avail_balance < cmd.amount {
        error!(
            "Rejected incoming command to withdraw funds. Insufficient funds in account {}",
            cmd.account_number
        );
        return vec![];
    } else {
        vec![Event::new(
            FUNDS_WITHDRAWN_EVENT_TYPE,
            STREAM,
            FundsWithdrawnEvent {
                account_number: cmd.account_number.to_string(),
                amount: cmd.amount,
                customer_id: old_state.customer_id,
                note: cmd.note,
            },
        )]
    }
}

fn deposit_funds(cmd: WithdrawFundsCommand) -> EventList {
    vec![Event::new(
        FUNDS_DEPOSITED_EVENT_TYPE,
        STREAM,
        FundsDepositedEvent {
            account_number: cmd.account_number.to_string(),
            amount: cmd.amount,
            customer_id: cmd.customer_id,
            note: cmd.note,
        },
    )]
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct AggregateState {
    // cents to avoid using float
    pub balance: u32,
    pub min_balance: u32,
    pub reserved_amount: u32,
    pub account_number: String,
    pub customer_id: String,
}

impl StateAck {
    fn ok(state: Option<AggregateState>) -> StateAck {
        StateAck {
            succeeded: true,
            error: None,
            state: state
                .clone()
                .map(|s| serde_json::to_vec(&s).unwrap_or_default()),
        }
    }

    fn error(msg: &str, state: Option<AggregateState>) -> StateAck {
        StateAck {
            succeeded: false,
            error: Some(msg.to_string()),
            state: state
                .clone()
                .map(|s| serde_json::to_vec(&s).unwrap_or_default()),
        }
    }
}

impl Event {
    fn new(event_type: &str, stream: &str, payload: impl Serialize) -> Event {
        Event {
            event_type: event_type.to_string(),
            stream: stream.to_string(),
            payload: serde_json::to_vec(&payload).unwrap_or_default(),
        }
    }
}
