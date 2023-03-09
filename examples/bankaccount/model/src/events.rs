use serde::{Serialize, Deserialize};

pub const ACCOUNT_CREATED_TYPE: &str = "account_created";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountCreatedEvent {
    pub initial_balance: u32,
    pub account_number: String,
    pub min_balance: u32
}