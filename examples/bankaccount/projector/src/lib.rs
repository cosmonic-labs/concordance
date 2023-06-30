use eventsourcing::*;
use genimpl::BankaccountProjectorImpl;
use wasmbus_rpc::actor::prelude::*;

use bankaccount_model::commands::*;
use bankaccount_model::events::*;
use bankaccount_model::state::*;
use serde::{Deserialize, Serialize};
use wasmcloud_interface_logging::debug;

#[allow(dead_code)]
mod eventsourcing;

#[allow(dead_code)]
mod genimpl;

#[allow(dead_code)]
mod system_traits;

mod store;

use system_traits::*;

impl BankaccountProjector for BankaccountProjectorImpl {
    fn handle_account_created(&self, input: AccountCreated) -> RpcResult<()> {
        store::initialize_account(input)
    }

    fn handle_wire_funds_reserved(&self, input: WireFundsReserved) -> RpcResult<()> {
        store::record_funds_reserved(input)
    }

    fn handle_funds_withdrawn(&self, input: FundsWithdrawn) -> RpcResult<()> {
        store::record_withdrawal(input)
    }

    fn handle_wire_funds_released(&self, input: WireFundsReleased) -> RpcResult<()> {
        store::release_reserved_funds(input)
    }

    fn handle_interbank_transfer_initiated(
        &self,
        _input: InterbankTransferInitiated,
    ) -> RpcResult<()> {
        Ok(())
    }

    fn handle_wire_transfer_requested(&self, _input: WireTransferRequested) -> RpcResult<()> {
        Ok(())
    }

    fn handle_funds_deposited(&self, input: FundsDeposited) -> RpcResult<()> {
        store::record_deposit(input)
    }

    fn handle_reserved_funds_withdrawn(&self, _input: ReservedFundsWithdrawn) -> RpcResult<()> {
        // TODO
        Ok(())
    }
}
