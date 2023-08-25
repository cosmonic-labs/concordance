use crate::*;

pub(crate) fn handle_reserve_funds(
    input: ReserveFunds,
    state: Option<BankAccountAggregateState>,
) -> Result<EventList> {
    let Some(old_state) = state else {
        return Err(anyhow::anyhow!(
            "Rejected command to reserve funds. Account {} does not exist.",
            input.account_number
        ));
    };
    let avail_balance = old_state.available_balance();
    if input.amount as u32 > avail_balance {
        error!(
            "Rejecting command to reserve funds, account {} does not have sufficient funds. Available {}",
            &input.account_number, avail_balance
        );
        Ok(vec![])
    } else {
        Ok(vec![Event::new(
            FundsReserved::TYPE,
            STREAM,
            &FundsReserved {
                account_number: input.account_number.to_string(),
                wire_transfer_id: input.wire_transfer_id,
                customer_id: old_state.customer_id.to_string(),
                amount: input.amount,
            },
        )])
    }
}

pub(crate) fn handle_release_funds(
    input: ReleaseFunds,
    state: Option<BankAccountAggregateState>,
) -> Result<EventList> {
    let Some(old_state) = state else {
        return Err(anyhow::anyhow!(
            "Rejected command to release funds. Account {} does not exist.",
            input.account_number
        ));
    };

    if old_state
        .reserved_funds
        .contains_key(&input.wire_transfer_id)
    {
        Ok(vec![Event::new(
            FundsReleased::TYPE,
            STREAM,
            &FundsReleased {
                customer_id: input.customer_id,
                account_number: input.account_number.to_string(),
                wire_transfer_id: input.wire_transfer_id.to_string(),
            },
        )])
    } else {
        error!(
                "Rejecting command to release funds, account {} does not have a wire transfer hold for {}",
                &input.account_number, input.wire_transfer_id
            );
        Ok(vec![])
    }
}

pub(crate) fn handle_commit_funds(
    input: CommitFunds,
    state: Option<BankAccountAggregateState>,
) -> Result<EventList> {
    let Some(old_state) = state else {
        return Err(anyhow::anyhow!(
            "Rejected command to commit funds. Account {} does not exist.",
            input.account_number
        ));
    };

    if old_state
        .reserved_funds
        .contains_key(&input.wire_transfer_id)
    {
        Ok(vec![Event::new(
            FundsCommitted::TYPE,
            STREAM,
            &FundsCommitted {
                customer_id: input.customer_id,
                account_number: input.account_number.to_string(),
                wire_transfer_id: input.wire_transfer_id.to_string(),
            },
        )])
    } else {
        error!(
                "Rejecting command to commit funds, account {} does not have a wire transfer hold for {}",
                &input.account_number, input.wire_transfer_id
            );
        Ok(vec![])
    }
}

pub(crate) fn handle_create_account(input: CreateAccount) -> Result<EventList> {
    Ok(vec![Event::new(
        AccountCreated::TYPE,
        STREAM,
        &AccountCreated {
            initial_balance: input.initial_balance,
            account_number: input.account_number.to_string(),
            min_balance: input.min_balance,
            customer_id: input.customer_id,
        },
    )])
}

pub(crate) fn handle_withdraw_funds(
    input: WithdrawFunds,
    state: Option<BankAccountAggregateState>,
) -> Result<EventList> {
    let Some(state) = state else {
        return Err(anyhow::anyhow!(
            "Rejected command to withdraw funds. Account {} does not exist.",
            input.account_number
        ));
    };

    if state.available_balance() < input.amount as u32 {
        error!(
                "Rejecting command to withdraw funds, account {} does not have sufficient funds. Available {}",
                &input.account_number, state.available_balance()
            );
        Ok(vec![])
    } else {
        Ok(vec![Event::new(
            FundsWithdrawn::TYPE,
            STREAM,
            &FundsWithdrawn {
                note: input.note,
                account_number: input.account_number.to_string(),
                amount: input.amount,
                customer_id: input.customer_id,
            },
        )])
    }
}

pub(crate) fn handle_wire_funds(
    input: WireFunds,
    state: Option<BankAccountAggregateState>,
) -> Result<EventList> {
    let Some(state) = state else {
        return Err(anyhow::anyhow!(
            "Rejected command to wire funds. Account {} does not exist.",
            input.account_number
        ));
    };

    if state.available_balance() < input.amount as u32 {
        error!(
                "Rejecting command to wire funds, account {} does not have sufficient funds. Available {}",
                &input.account_number, state.available_balance()
            );
        Ok(vec![])
    } else {
        Ok(vec![Event::new(
            WireTransferInitiated::TYPE,
            STREAM,
            &WireTransferInitiated {
                note: input.note,
                account_number: input.target_account_number,
                target_routing_number: input.target_routing_number,
                target_account_number: input.account_number,
                amount: input.amount,
                customer_id: input.customer_id,
                wire_transfer_id: input.wire_transaction_id,
            },
        )])
    }
}

pub(crate) fn handle_deposit_funds(
    input: DepositFunds,
    state: Option<BankAccountAggregateState>,
) -> Result<EventList> {
    if state.is_none() {
        return Err(anyhow::anyhow!(
            "Rejected command to deposit funds. Account {} does not exist.",
            input.account_number
        ));
    };

    Ok(vec![Event::new(
        FundsDeposited::TYPE,
        STREAM,
        &FundsDeposited {
            note: input.note,
            account_number: input.account_number.to_string(),
            amount: input.amount,
            customer_id: input.customer_id,
        },
    )])
}
