use anyhow::Result;
use serde::{Deserialize, Serialize};

concordance_gen::generate!({
    path: "../eventcatalog",
    role: "projector",
    entity: "bank account"
});

mod store;

#[async_trait]
impl BankAccountProjector for BankAccountProjectorImpl {
    async fn handle_account_created(&self, input: AccountCreated) -> Result<()> {
        store::initialize_account(input).await
    }

    async fn handle_funds_deposited(&self, input: FundsDeposited) -> Result<()> {
        store::record_funds_deposited(input).await
    }

    async fn handle_funds_reserved(&self, input: FundsReserved) -> Result<()> {
        store::record_funds_reserved(input).await
    }

    async fn handle_funds_withdrawn(&self, input: FundsWithdrawn) -> Result<()> {
        store::record_funds_withdrawn(input).await
    }

    async fn handle_funds_released(&self, input: FundsReleased) -> Result<()> {
        store::record_funds_released(input).await
    }

    async fn handle_wire_transfer_initiated(&self, _input: WireTransferInitiated) -> Result<()> {
        Ok(())
    }
}

