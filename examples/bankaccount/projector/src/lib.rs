use bankaccount_model::events::*;

use serde::{Deserialize, Serialize};

concordance_gen::generate!({
    path: "../bankaccount-model.ttl",
    role: "projector",
    entity: "bankaccount"
});

mod store;

#[async_trait]
impl BankaccountProjector for BankaccountProjectorImpl {
    async fn handle_account_created(&self, input: AccountCreated) -> RpcResult<()> {
        store::initialize_account(input).await
    }

    async fn handle_wire_funds_reserved(&self, input: WireFundsReserved) -> RpcResult<()> {
        store::record_funds_reserved(input).await
    }

    async fn handle_funds_withdrawn(&self, input: FundsWithdrawn) -> RpcResult<()> {
        store::record_withdrawal(input).await
    }

    async fn handle_wire_funds_released(&self, input: WireFundsReleased) -> RpcResult<()> {
        store::release_reserved_funds(input).await
    }

    async fn handle_interbank_transfer_initiated(
        &self,
        _input: InterbankTransferInitiated,
    ) -> RpcResult<()> {
        Ok(())
    }

    async fn handle_wire_transfer_requested(&self, _input: WireTransferRequested) -> RpcResult<()> {
        Ok(())
    }

    async fn handle_funds_deposited(&self, input: FundsDeposited) -> RpcResult<()> {
        store::record_deposit(input).await
    }

    async fn handle_reserved_funds_withdrawn(
        &self,
        _input: ReservedFundsWithdrawn,
    ) -> RpcResult<()> {
        // TODO
        Ok(())
    }
}
