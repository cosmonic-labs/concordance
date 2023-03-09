use serde::{Deserialize, Serialize};

pub const CREATE_ACCOUNT_TYPE: &str = "create_account";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountCommand {
    pub account_number: String,
    pub min_balance: u32,
    pub initial_balance: u32,
    pub customer_id: String,
}
