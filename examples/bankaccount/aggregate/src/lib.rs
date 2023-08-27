use anyhow::Result;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wasmcloud_interface_logging::error;

mod commands;
mod events;
mod state;

use state::BankAccountAggregateState;

concordance_gen::generate!({
    path: "../eventcatalog",
    role: "aggregate",
    entity: "bank account"
});

impl BankAccountAggregate for BankAccountAggregateImpl {
    // -- Commands --
    fn handle_reserve_funds(
        &self,
        input: ReserveFunds,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_reserve_funds(input, state)
    }

    fn handle_release_funds(
        &self,
        input: ReleaseFunds,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_release_funds(input, state)
    }

    fn handle_commit_funds(
        &self,
        input: CommitFunds,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_commit_funds(input, state)
    }

    fn handle_create_account(
        &self,
        input: CreateAccount,
        _state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_create_account(input)
    }

    fn handle_withdraw_funds(
        &self,
        input: WithdrawFunds,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_withdraw_funds(input, state)
    }

    fn handle_wire_funds(
        &self,
        input: WireFunds,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_wire_funds(input, state)
    }

    fn handle_deposit_funds(
        &self,
        input: DepositFunds,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_deposit_funds(input, state)
    }

    // -- Events --

    fn apply_account_created(
        &self,
        input: AccountCreated,
        _state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_account_created(input)
    }

    fn apply_funds_deposited(
        &self,
        input: FundsDeposited,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_funds_deposited(input, state)
    }

    fn apply_funds_released(
        &self,
        input: FundsReleased,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_funds_released(input, state)
    }

    fn apply_funds_committed(
        &self,
        input: FundsCommitted,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_funds_committed(input, state)
    }

    fn apply_funds_reserved(
        &self,
        input: FundsReserved,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_funds_reserved(input, state)
    }

    fn apply_funds_withdrawn(
        &self,
        input: FundsWithdrawn,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_funds_withdrawn(input, state)
    }

    fn apply_wire_transfer_initiated(
        &self,
        input: WireTransferInitiated,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_wire_transfer_initiated(input, state)
    }
}

const STREAM: &str = "bankaccount";
