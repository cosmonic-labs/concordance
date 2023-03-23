use serde::{Deserialize, Serialize};

/// Requests the creation of a new bank account with the metadata contained
/// in the command. Realistically consumers should assume that a command
/// to create an already-existing account will be rejected/ignored
pub const CREATE_ACCOUNT_TYPE: &str = "create_account";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountCommand {
    pub account_number: String,
    pub min_balance: u32,
    pub initial_balance: u32,
    pub customer_id: String,
}


/// Requests the withdrawal of funds from the account. A real request would have much
/// more metadata on it, but in this case we use the `note` field to indicate the nature of
/// the withdrawal, e.g. "ATM withdrawal" or "teller withdrawal", etc
pub const WITHDRAW_FUNDS_TYPE: &str = "withdraw_funds";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawFundsCommand {
    pub account_number: String,
    pub amount: u32,
    pub note: String,
    pub customer_id: String,
}

/// Requests the deposit of funds into the account. The deposit is a "simple" deposit and
/// the nature of the deposit (e.g. "cash deposit", "teller deposit", etc) will be recorded
/// in the `note` field.
pub const DEPOSIT_FUNDS_TYPE: &str = "deposit_funds";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositFundsCommand {
    pub account_number: String,
    pub amount: u32,
    pub note: String,
    pub customer_id: String
}

/// Requests the simple transfer of funds between accounts owned by the same customer
pub const TRANSFER_FUNDS_TYPE: &str = "transfer_funds";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferFundsCommand {
    pub source_account_number: String,
    pub dest_account_number: String,
    pub amount: u32,
    pub note: String,
}

/// Requests a transfer of funds between banks, with the `account_number` representing
/// the source account and the target fields identifying the destination of the funds
/// transfer. Events produced by the processing of this command result in the generation 
/// of multi-step processes managed by the process manager for interbank transfers
pub const WIRE_TRANSFER_REQUEST_TYPE: &str = "request_wire_transfer";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestWireTransfer {
    pub account_number: String,
    pub amount: u32,
    pub note: String,
    pub target_routing_number: String,
    pub target_account_number: String,    
    pub wire_transfer_id: String
}

/// Reserves funds in a target account with regard to a given wire transfer transaction
/// ID. Reserved funds will not change an account's balance but those funds will be unavailable
/// as part of the adjusted balance used for validating other commands
pub const RESERVE_FUNDS_TYPE: &str = "reserve_funds";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReserveFunds {
    pub account_number: String,
    pub amount: u32,
    pub wire_transfer_id: String,
}

/// A request to initiate an interbank wire transfer. This can only happen once the funds
/// for this transfer have been reserved
pub const INITIATE_TRANSFER_TYPE: &str = "initiate_wire_transfer";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateInterbankTransfer {
    pub account_number: String,
    pub amount: u32,
    pub wire_transfer_id: String,
    pub expiration_in_days: u32,
    pub target_routing_number: String,
    pub target_account_number: String,
}


/// Upon successful completion of an interbank wire transfer, the previously reserved funds
/// will be withdrawn from the source account
pub const WITHDRAW_RESERVED_FUNDS_TYPE: &str = "withdraw_reserved_funds";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawReservedFunds {
    pub account_number: String,
    pub wire_transfer_id: String,
    pub amount: u32
}

/// If an interbank wire transfer fails, then this command will request the release of previously
/// reserved funds, once again making them available for other transactions
pub const RELEASE_RESERVED_FUNDS_TYPE: &str = "release_reserved_funds";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseReservedFunds {
    pub account_number: String,
    pub wire_transfer_id: String,
    pub amount: u32
}