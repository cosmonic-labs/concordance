use serde::{Deserialize, Serialize};

/// Requests the creation of a new bank account with the metadata contained
/// in the command. Realistically consumers should assume that a command
/// to create an already-existing account will be rejected/ignored

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccount {
    pub account_number: String,
    pub min_balance: u32,
    pub initial_balance: u32,
    pub customer_id: String,
}

impl CreateAccount {
    pub const TYPE: &str = "create_account";
}

/// Requests the withdrawal of funds from the account. A real request would have much
/// more metadata on it, but in this case we use the `note` field to indicate the nature of
/// the withdrawal, e.g. "ATM withdrawal" or "teller withdrawal", etc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawFunds {
    pub account_number: String,
    pub amount: u32,
    pub note: String,
    pub customer_id: String,
}

impl WithdrawFunds {
    pub const TYPE: &str = "withdraw_funds";
}

/// Requests the deposit of funds into the account. The deposit is a "simple" deposit and
/// the nature of the deposit (e.g. "cash deposit", "teller deposit", etc) will be recorded
/// in the `note` field.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositFunds {
    pub account_number: String,
    pub amount: u32,
    pub note: String,
    pub customer_id: String,
}

impl DepositFunds {
    pub const TYPE: &str = "deposit_funds";
}

/// Requests the simple transfer of funds between accounts owned by the same customer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferFunds {
    pub source_account_number: String,
    pub dest_account_number: String,
    pub amount: u32,
    pub note: String,
}

impl TransferFunds {
    pub const TYPE: &str = "transfer_funds";
}

/// Requests a transfer of funds between banks, with the `account_number` representing
/// the source account and the target fields identifying the destination of the funds
/// transfer. Events produced by the processing of this command result in the generation
/// of multi-step processes managed by the process manager for interbank transfers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestWireTransfer {
    pub account_number: String,
    pub amount: u32,
    pub note: String,
    pub target_routing_number: String,
    pub target_account_number: String,
    pub wire_transfer_id: String,
}

impl RequestWireTransfer {
    pub const TYPE: &str = "request_wire_transfer";
}

/// Reserves funds in a target account with regard to a given wire transfer transaction
/// ID. Reserved funds will not change an account's balance but those funds will be unavailable
/// as part of the adjusted balance used for validating other commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReserveFunds {
    pub account_number: String,
    pub amount: u32,
    pub wire_transfer_id: String,
}

impl ReserveFunds {
    pub const TYPE: &str = "reserve_funds";
}

/// A request to initiate an interbank wire transfer. This can only happen once the funds
/// for this transfer have been reserved
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateInterbankTransfer {
    pub account_number: String,
    pub amount: u32,
    pub wire_transfer_id: String,
    pub expiration_in_days: u32,
    pub target_routing_number: String,
    pub target_account_number: String,
}

impl InitiateInterbankTransfer {
    pub const TYPE: &str = "initiate_wire_transfer";
}

/// Upon successful completion of an interbank wire transfer, the previously reserved funds
/// will be withdrawn from the source account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawReservedFunds {
    pub account_number: String,
    pub wire_transfer_id: String,
    pub amount: u32,
}

impl WithdrawReservedFunds {
    pub const TYPE: &str = "withdraw_reserved_funds";
}

/// If an interbank wire transfer fails, then this command will request the release of previously
/// reserved funds, once again making them available for other transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseReservedFunds {
    pub account_number: String,
    pub wire_transfer_id: String,
    pub amount: u32,
}

impl ReleaseReservedFunds {
    pub const TYPE: &str = "release_reserved_funds";
}
