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