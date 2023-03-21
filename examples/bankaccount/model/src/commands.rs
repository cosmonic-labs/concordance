use serde::{Deserialize, Serialize};

pub const CREATE_ACCOUNT_TYPE: &str = "create_account";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountCommand {
    pub account_number: String,
    pub min_balance: u32,
    pub initial_balance: u32,
    pub customer_id: String,
}


pub const WITHDRAW_FUNDS_TYPE: &str = "withdraw_funds";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawFundsCommand {
    pub account_number: String,
    pub amount: u32,
    pub note: String,
    pub customer_id: String,
}

pub const DEPOSIT_FUNDS_TYPE: &str = "deposit_funds";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositFundsCommand {
    pub account_number: String,
    pub amount: u32,
    pub note: String,
    pub customer_id: String
}

pub const TRANSFER_FUNDS_TYPE: &str = "transfer_funds";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferFundsCommand {
    pub source_account_number: String,
    pub dest_account_number: String,
    pub amount: u32,
    pub note: String,
}