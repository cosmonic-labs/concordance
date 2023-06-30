use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountCreated {
    pub initial_balance: u32,
    pub account_number: String,
    pub min_balance: u32,
    pub customer_id: String,
}

impl AccountCreated {
    pub const TYPE: &str = "account_created";
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FundsWithdrawn {
    pub account_number: String,
    pub amount: u32,
    pub customer_id: String,
    pub note: String,
}

impl FundsWithdrawn {
    pub const TYPE: &str = "funds_withdrawn";
}

/// After a wire transfer is complete, reserved funds are withdrawn
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReservedFundsWithdrawn {
    pub account_number: String,
    pub wire_transfer_id: String,
    pub amount: u32,
}

impl ReservedFundsWithdrawn {
    pub const TYPE: &str = "reserved_funds_withdrawn";
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FundsDeposited {
    pub account_number: String,
    pub amount: u32,
    pub customer_id: String,
    pub note: String,
}

impl FundsDeposited {
    pub const TYPE: &str = "funds_deposited";
}

/* Bank Account Interbank Xfer Process Manager */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WireTransferRequested {
    pub wire_transfer_id: String,
    pub account_number: String,
    pub customer_id: String,
    pub amount: u32,
    pub target_routing_number: String,
    pub target_account_number: String,
}

impl WireTransferRequested {
    pub const TYPE: &str = "wire_transfer_requested";
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WireFundsReserved {
    pub account_number: String,
    pub wire_transfer_id: String,
    pub customer_id: String,
    pub amount: u32,
}

impl WireFundsReserved {
    pub const TYPE: &str = "wire_funds_reserved";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireFundsReleased {
    pub account_number: String,
    pub wire_transfer_id: String,
    pub amount: u32,
}

impl WireFundsReleased {
    pub const TYPE: &str = "wire_funds_released";
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InterbankTransferInitiated {
    pub account_number: String,
    pub wire_transfer_id: String,
    pub target_routing_number: String,
    pub target_account_number: String,
}

impl InterbankTransferInitiated {
    pub const TYPE: &str = "interbank_transfer_initiated";
}

/* Events from the interbank gateway */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InterbankTransferCompleted {
    pub wire_transfer_id: String,
    pub note: String,
    pub gateway_client_id: String,
}

impl InterbankTransferCompleted {
    pub const TYPE: &str = "interbank_transfer_completed";
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InterbankTransferFailed {
    pub wire_transfer_id: String,
    pub note: String,
    pub gateway_client_id: String,
}

impl InterbankTransferFailed {
    pub const TYPE: &str = "interbank_transfer_failed";
}
