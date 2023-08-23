use serde::{Deserialize, Serialize};
use wasmcloud_interface_logging::{debug, error};

//use lunarfrontiers_model::*;
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct BankAccountAggregateState {
    pub placeholder: u16,
}

concordance_gen::generate!({
    path: "../eventcatalog",
    role: "aggregate",
    entity: "bank account"
});

impl BankAccountAggregate for BankAccountAggregateImpl {
    fn handle_reserve_funds(
        &self,
        input: ReserveFunds,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        todo!()
    }

    fn handle_release_funds(
        &self,
        input: ReleaseFunds,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        todo!()
    }

    fn handle_commit_funds(
        &self,
        input: CommitFunds,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        todo!()
    }

    fn handle_create_account(
        &self,
        input: CreateAccount,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        todo!()
    }

    fn handle_withdraw_funds(
        &self,
        input: WithdrawFunds,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        todo!()
    }

    fn handle_wire_funds(
        &self,
        input: WireFunds,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        todo!()
    }

    fn handle_deposit_funds(
        &self,
        input: DepositFunds,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        todo!()
    }

    fn apply_account_created(
        &self,
        input: AccountCreated,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        todo!()
    }

    fn apply_funds_deposited(
        &self,
        input: FundsDeposited,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        todo!()
    }

    fn apply_funds_released(
        &self,
        input: FundsReleased,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        todo!()
    }

    fn apply_funds_committed(
        &self,
        input: FundsCommitted,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        todo!()
    }

    fn apply_funds_reserved(
        &self,
        input: FundsReserved,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        todo!()
    }

    fn apply_funds_withdrawn(
        &self,
        input: FundsWithdrawn,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        todo!()
    }

    fn apply_wire_transfer_initiated(
        &self,
        input: WireTransferInitiated,
        state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        todo!()
    }
}

const STREAM: &str = "bankaccount";
/*
impl BankaccountAggregate for BankaccountAggregateImpl {
    /* --- Command Handlers --- */

    fn handle_create_account(
        &self,
        cmd: CreateAccount,
        _state: Option<BankaccountAggregateState>,
    ) -> Result<EventList> {
        Ok(vec![Event::new(
            AccountCreated::TYPE,
            STREAM,
            &AccountCreated {
                initial_balance: cmd.initial_balance,
                account_number: cmd.account_number.to_string(),
                min_balance: cmd.min_balance,
                customer_id: cmd.customer_id,
            },
        )])
    }

    fn handle_reserve_funds(
        &self,
        cmd: ReserveFunds,
        state: Option<BankaccountAggregateState>,
    ) -> Result<EventList> {
        let Some(old_state) = state else {
            error!(
                "Rejected incoming command to reserve funds. Account {} does not exist.",
                cmd.account_number
            );
            return Ok(vec![]);
        };
        let avail_balance = old_state.balance - old_state.reserved_amount;
        if cmd.amount > avail_balance {
            // In a real-world system this might emit a failure event rather than just silently absorb the command
            error!(
                "Rejecting command to reserve funds, account {} does not have sufficient funds.",
                &cmd.account_number
            );
            Ok(vec![])
        } else {
            Ok(vec![Event::new(
                WireFundsReserved::TYPE,
                STREAM,
                WireFundsReserved {
                    account_number: cmd.account_number.to_string(),
                    wire_transfer_id: cmd.wire_transfer_id,
                    customer_id: old_state.customer_id.to_string(),
                    amount: cmd.amount,
                },
            )])
        }
    }
    fn handle_release_reserved_funds(
        &self,
        cmd: ReleaseReservedFunds,
        state: Option<BankaccountAggregateState>,
    ) -> Result<EventList> {
        let Some(old_state) = state else {
            error!(
                "Rejected incoming command to release reserved funds. Account {} does not exist.",
                cmd.account_number
            );
            return Ok(vec![]);
        };
        let adj_amount = cmd.amount.min(old_state.balance);
        Ok(vec![Event::new(
            WireFundsReleased::TYPE,
            STREAM,
            WireFundsReleased {
                account_number: cmd.account_number.to_string(),
                wire_transfer_id: cmd.wire_transfer_id.to_string(),
                amount: adj_amount,
            },
        )])
    }
    fn handle_withdraw_funds(
        &self,
        cmd: WithdrawFunds,
        state: Option<BankaccountAggregateState>,
    ) -> Result<EventList> {
        let Some(old_state) = state else {
            error!(
                "Rejected incoming command to withdraw funds. Account {} does not exist.",
                cmd.account_number
            );
            return Ok(vec![]);
        };

        let avail_balance = old_state.balance - old_state.reserved_amount;
        if avail_balance < cmd.amount {
            error!(
                "Rejected incoming command to withdraw funds. Insufficient funds in account {}",
                cmd.account_number
            );
            return Ok(vec![]);
        } else {
            Ok(vec![Event::new(
                FundsWithdrawn::TYPE,
                STREAM,
                FundsWithdrawn {
                    account_number: cmd.account_number.to_string(),
                    amount: cmd.amount,
                    customer_id: old_state.customer_id,
                    note: cmd.note,
                },
            )])
        }
    }

    fn handle_withdraw_reserved_funds(
        &self,
        input: WithdrawReservedFunds,
        state: Option<BankaccountAggregateState>,
    ) -> Result<EventList> {
        if state.is_none() {
            error!(
                "Rejecting withdraw reserved funds command. Account {} does not exist.",
                input.account_number
            );
            return Ok(vec![]);
        };

        Ok(vec![Event::new(
            ReservedFundsWithdrawn::TYPE,
            STREAM,
            ReservedFundsWithdrawn {
                account_number: input.account_number.to_string(),
                wire_transfer_id: input.wire_transfer_id.to_string(),
                amount: input.amount,
            },
        )])
    }

    fn handle_deposit_funds(
        &self,
        cmd: DepositFunds,
        state: Option<BankaccountAggregateState>,
    ) -> Result<EventList> {
        let Some(old_state) = state else {
            error!(
                "Rejected incoming command to withdraw funds. Account {} does not exist.",
                cmd.account_number
            );
            return Ok(vec![]);
        };

        let avail_balance = old_state.balance - old_state.reserved_amount;
        if avail_balance < cmd.amount {
            error!(
                "Rejected incoming command to withdraw funds. Insufficient funds in account {}",
                cmd.account_number
            );
            return Ok(vec![]);
        } else {
            Ok(vec![Event::new(
                FundsWithdrawn::TYPE,
                STREAM,
                FundsWithdrawn {
                    account_number: cmd.account_number.to_string(),
                    amount: cmd.amount,
                    customer_id: old_state.customer_id,
                    note: cmd.note,
                },
            )])
        }
    }
    fn handle_request_wire_transfer(
        &self,
        cmd: RequestWireTransfer,
        state: Option<BankaccountAggregateState>,
    ) -> Result<EventList> {
        let Some(old_state) = state else {
            error!(
                "Rejected incoming command to request a wire transfer. Account {} does not exist.",
                cmd.account_number
            );
            return Ok(vec![]);
        };

        Ok(vec![Event::new(
            WireTransferRequested::TYPE,
            STREAM,
            WireTransferRequested {
                account_number: cmd.account_number.to_string(),
                wire_transfer_id: cmd.wire_transfer_id,
                amount: cmd.amount,
                customer_id: old_state.customer_id,
                target_account_number: cmd.target_account_number,
                target_routing_number: cmd.target_routing_number,
            },
        )])
    }
    fn handle_initiate_interbank_transfer(
        &self,
        cmd: InitiateInterbankTransfer,
        state: Option<BankaccountAggregateState>,
    ) -> Result<EventList> {
        if state.is_none() {
            error!(
                "Rejected incoming command to initiate bank transfer. Account {} does not exist.",
                cmd.account_number
            );
            return Ok(vec![]);
        };
        // NOTE: validation would occur here in a real app
        Ok(vec![Event::new(
            InterbankTransferInitiated::TYPE,
            STREAM,
            InterbankTransferInitiated {
                account_number: cmd.account_number.to_string(),
                wire_transfer_id: cmd.wire_transfer_id,
                target_account_number: cmd.target_account_number,
                target_routing_number: cmd.target_routing_number,
            },
        )])
    }

    /* --- Event Appliers --- */

    /// Update state with new account
    fn apply_account_created(
        &self,
        input: AccountCreated,
        _state: Option<BankaccountAggregateState>,
    ) -> Result<StateAck> {
        let state = BankaccountAggregateState {
            balance: input.initial_balance,
            min_balance: input.min_balance,
            account_number: input.account_number,
            customer_id: input.customer_id,
            reserved_amount: 0,
        };
        Ok(StateAck::ok(Some(state)))
    }

    /// Add deposited funds to balance
    fn apply_funds_deposited(
        &self,
        input: FundsDeposited,
        state: Option<BankaccountAggregateState>,
    ) -> Result<StateAck> {
        let Some(old_state) = state else {
            error!(
                "Rejecting funds deposited event. Account {} does not exist.",
                input.account_number
            );
            return Ok(StateAck::error("Account does not exist", None::<BankaccountAggregateState>));
        };
        let state = BankaccountAggregateState {
            balance: old_state.balance + input.amount,
            ..old_state
        };
        Ok(StateAck::ok(Some(state)))
    }

    /// Put reserved funds on hold
    fn apply_wire_funds_reserved(
        &self,
        input: WireFundsReserved,
        state: Option<BankaccountAggregateState>,
    ) -> Result<StateAck> {
        let Some(old_state) = state else {
            error!(
                "Rejecting funds reserved event. Account {} does not exist.",
                input.account_number
            );
            return Ok(StateAck::error("Account does not exist", None::<BankaccountAggregateState>));
        };

        let state = BankaccountAggregateState {
            reserved_amount: old_state.reserved_amount + input.amount,
            ..old_state
        };
        Ok(StateAck::ok(Some(state)))
    }

    /// Withdraw funds from balance
    fn apply_funds_withdrawn(
        &self,
        input: FundsWithdrawn,
        state: Option<BankaccountAggregateState>,
    ) -> Result<StateAck> {
        let Some(old_state) = state else {
            error!(
                "Rejecting funds withdrawn event. Account {} does not exist.",
                input.account_number
            );
            return Ok(StateAck::error("Account does not exist", None::<BankaccountAggregateState>));
        };

        let state = BankaccountAggregateState {
            balance: old_state.balance - input.amount,
            ..old_state
        };
        Ok(StateAck::ok(Some(state)))
    }

    /// Release previously reserved funds
    fn apply_wire_funds_released(
        &self,
        input: WireFundsReleased,
        state: Option<BankaccountAggregateState>,
    ) -> Result<StateAck> {
        let Some(old_state) = state else {
            error!(
                "Rejecting funds reserved event. Account {} does not exist.",
                input.account_number
            );
            return Ok(StateAck::error("Account does not exist", None::<BankaccountAggregateState>));
        };

        let state = BankaccountAggregateState {
            reserved_amount: old_state.reserved_amount - input.amount,
            ..old_state
        };
        Ok(StateAck::ok(Some(state)))
    }

    /// Initiate an interbank transfer (which in turn triggers an external process)
    fn apply_interbank_transfer_initiated(
        &self,
        _input: InterbankTransferInitiated,
        state: Option<BankaccountAggregateState>,
    ) -> Result<StateAck> {
        // This is a no-op
        Ok(StateAck::ok(state))
    }

    /// Update state with wire transfer request
    fn apply_wire_transfer_requested(
        &self,
        _input: WireTransferRequested,
        state: Option<BankaccountAggregateState>,
    ) -> Result<StateAck> {
        // currently a no-op
        Ok(StateAck::ok(state))
    }

    fn apply_reserved_funds_withdrawn(
        &self,
        input: ReservedFundsWithdrawn,
        state: Option<BankaccountAggregateState>,
    ) -> Result<StateAck> {
        let Some(old_state) = state else {
            error!(
                "Rejecting funds reserved event. Account {} does not exist.",
                input.account_number
            );
            return Ok(StateAck::error("Account does not exist", None::<BankaccountAggregateState>));
        };

        // withdraw the amount on reserve, set amount on reserve to 0
        let state = BankaccountAggregateState {
            reserved_amount: 0,
            balance: old_state.balance - input.amount,
            ..old_state
        };
        Ok(StateAck::ok(Some(state)))
    }
}
*/
