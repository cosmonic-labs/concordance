use serde::{Serialize, Deserialize};

pub const ACCOUNT_CREATED_TYPE: &str = "account_created";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountCreatedEvent {
    pub initial_balance: u32,
    pub account_number: String,
    pub min_balance: u32,
    pub customer_id: String,
}

pub const FUNDS_WITHDRAWN_EVENT_TYPE: &str = "funds_withdrawn";
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FundsWithdrawnEvent {
    pub account_number: String,
    pub amount: u32,
    pub customer_id: String,
    pub note: String,
}

pub const FUNDS_DEPOSITED_EVENT_TYPE: &str = "funds_deposited";
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FundsDepositedEvent {
    pub account_number: String,
    pub amount: u32,
    pub customer_id: String,
    pub note: String,
}


/* Bank Account Interbank Xfer Process Manager */

pub const WIRE_TRANSFER_REQUESTED_EVENT_TYPE: &str = "wire_transfer_requested";
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WireTransferRequested {
    pub wire_transfer_id: String,
    pub account_number: String,    
    pub customer_id: String,
    pub amount: u32,
    pub target_routing_number: String,
    pub target_account_number: String,
}

pub const WIRE_FUNDS_RESERVED_EVENT_TYPE: &str = "wire_funds_reserved";
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WireFundsReserved {
    pub account_number: String,
    pub wire_transfer_id: String,
    pub customer_id: String,
    pub amount: u32
}

pub const INTERBANK_TRANSFER_INITIATED_EVENT_TYPE: &str = "interbank_transfer_initiated";
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InterbankTransferInitiated {
    pub account_number: String,
    pub wire_transfer_id: String,
    pub target_routing_number: String,
    pub target_account_number: String,
}

/* Events from the interbank gateway */

pub const INTERBANK_TRANSFER_COMPLETED_EVENT_TYPE: &str = "interbank_transfer_completed";
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InterbankTransferCompleted {
    pub wire_transfer_id: String,
    pub note: String,
    pub gateway_client_id: String,
}

pub const INTERBANK_TRANSFER_FAILED_EVENT_TYPE: &str = "interbank_transfer_failed";
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InterbankTransferFailed {
    pub wire_transfer_id: String,
    pub note: String,
    pub gateway_client_id: String
}


