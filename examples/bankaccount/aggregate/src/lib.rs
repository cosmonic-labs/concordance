use eventsourcing::*;
use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;

use bankaccount_model::commands::{CreateAccountCommand, CREATE_ACCOUNT_TYPE};
use bankaccount_model::events::{AccountCreatedEvent, ACCOUNT_CREATED_TYPE};
use bankaccount_model::{deserialize, serialize};

#[allow(dead_code)]
mod eventsourcing;

const STREAM: &str = "bankaccount";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, AggregateService)]
struct BankAccountAggregate {}

#[async_trait]
impl AggregateService for BankAccountAggregate {
    async fn handle_command(&self, _ctx: &Context, arg: &StatefulCommand) -> RpcResult<EventList> {
        Ok(match arg.command_type.as_str() {
            CREATE_ACCOUNT_TYPE => create_account(deserialize(&arg.payload).unwrap()),
            _ => {
                vec![]
            }
        })
    }

    async fn apply_event(&self, _ctx: &Context, arg: &EventWithState) -> RpcResult<StateAck> {
        let old_state: AggregateState = if let Some(s) = &arg.state {
            deserialize(&s).unwrap()
        } else {
            AggregateState::default()
        };
        let new_state = match arg.event.event_type.as_str() {
            ACCOUNT_CREATED_TYPE => apply_account_created(deserialize(&arg.event.payload).unwrap()),
            _ => old_state,
        };
        Ok(StateAck {
            error: None,
            state: serialize(&new_state).unwrap(),
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
}
