use eventsourcing::*;
use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;

use bankaccount_model::commands::*;
use bankaccount_model::events::*;
use bankaccount_model::{deserialize, serialize};
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
        info!("Handling command: {}", arg.command_type.as_str());
        Ok(match arg.command_type.as_str() {
            CREATE_ACCOUNT_TYPE => create_account(
                deserialize(&arg.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            DEPOSIT_FUNDS_TYPE => deposit_funds(
                deserialize(&arg.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            WITHDRAW_FUNDS_TYPE => withdraw_funds(
                arg.state.clone(),
                deserialize(&arg.payload).map_err(|e| RpcError::Deser(e.to_string()))?,
            ),
            e => {
                error!("Unsupported command type: {e}. Interest configuration for this aggregate is probably incorect.");
                vec![]
            }
        })
    }

    async fn apply_event(&self, _ctx: &Context, arg: &EventWithState) -> RpcResult<StateAck> {
        info!("Handling event {}", arg.event.event_type);
        let old_state: AggregateState = if let Some(s) = &arg.state {
            deserialize(&s).unwrap()
        } else {
            AggregateState::default()
        };
        let new_state = match arg.event.event_type.as_str() {
            ACCOUNT_CREATED_TYPE => apply_account_created(deserialize(&arg.event.payload).unwrap()),
            FUNDS_DEPOSITED_EVENT_TYPE => apply_funds_deposited(deserialize(&arg.event.payload).unwrap(), old_state),
            FUNDS_WITHDRAWN_EVENT_TYPE => apply_funds_withdrawn(deserialize(&arg.event.payload).unwrap(), old_state),
            _ => old_state,
        };
        Ok(StateAck {
            error: None,
            state: Some(serialize(&new_state).unwrap()),
            succeeded: true,
        })
    }
}

// This function doesn't use/care about pre-existing state. This creates it new
fn apply_account_created(event: AccountCreatedEvent) -> AggregateState {
    AggregateState {
        balance: event.initial_balance,
        min_balance: event.min_balance,
        account_number: event.account_number,
        customer_id: event.customer_id,
    }
}

fn apply_funds_deposited(event: FundsDepositedEvent, state: AggregateState) -> AggregateState {
    AggregateState { 
        balance: state.balance + event.amount,
        .. state
    }
}

fn apply_funds_withdrawn(event: FundsWithdrawnEvent, state: AggregateState) -> AggregateState {
    AggregateState { 
        balance: state.balance - event.amount,
        .. state
    }
}

fn create_account(cmd: CreateAccountCommand) -> EventList {
    vec![Event {
        event_type: ACCOUNT_CREATED_TYPE.to_string(),
        key: cmd.account_number.to_string(),
        payload: serialize(&AccountCreatedEvent {
            initial_balance: cmd.initial_balance,
            account_number: cmd.account_number,
            min_balance: cmd.min_balance,
            customer_id: cmd.customer_id,
        })
        .unwrap(),
        stream: STREAM.to_string(),
    }]
}

fn withdraw_funds(state: Option<Vec<u8>>, cmd: WithdrawFundsCommand) -> EventList {
    // TODO: enforce minimum balance
    if let Some(s) = state {
        let Ok(old_state) = deserialize::<AggregateState>(&s) else {
            error!("Rejected command to withdraw funds from account {}. Could not deserialize old state.", cmd.account_number);
            return vec![];
        };
        let adj_withdraw = old_state.balance.min(cmd.amount);
        vec![Event {
            event_type: FUNDS_WITHDRAWN_EVENT_TYPE.to_string(),
            key: cmd.account_number.to_string(),
            payload: serialize(&FundsWithdrawnEvent {
                account_number: cmd.account_number,
                amount: adj_withdraw,
                customer_id: cmd.customer_id,
                note: cmd.note,
            })
            .unwrap(),
            stream: STREAM.to_string(),
        }]
    } else {
        error!(
            "Rejected incoming command to withdraw funds. Account {} does not exist.",
            cmd.account_number
        );
        vec![]
    }
}

fn deposit_funds(cmd: WithdrawFundsCommand) -> EventList {
    vec![Event {
        event_type: FUNDS_DEPOSITED_EVENT_TYPE.to_string(),
        key: cmd.account_number.to_string(),
        payload: serialize(&FundsDepositedEvent {
            account_number: cmd.account_number,
            amount: cmd.amount,
            customer_id: cmd.customer_id,
            note: cmd.note,
        })
        .unwrap(),
        stream: STREAM.to_string(),
    }]
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct AggregateState {
    // cents to avoid using float
    pub balance: u32,
    pub min_balance: u32,
    pub account_number: String,
    pub customer_id: String,
}
