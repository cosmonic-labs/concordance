use serde::{Deserialize, Serialize};

use crate::events::WireTransferRequested;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct BankaccountAggregateState {
    // cents to avoid using float
    pub balance: u32,
    pub min_balance: u32,
    pub reserved_amount: u32,
    pub account_number: String,
    pub customer_id: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum TransferStatus {
    Requested,
    FundsReserved,
    TransferInitiated,
    TransferCompleted,
    TransferFailed,
    Unknown,
}

impl Default for TransferStatus {
    fn default() -> Self {
        TransferStatus::Unknown
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankaccountProcessManagerState {
    pub wire_transfer_id: String,
    pub account_number: String,
    pub customer_id: String,
    pub amount: u32,
    pub target_routing_number: String,
    pub target_account_number: String,
    pub status: TransferStatus,
}

impl BankaccountProcessManagerState {
    pub fn to_bytes(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap_or_default()
    }
}

impl BankaccountProcessManagerState {
    pub fn new(event: &WireTransferRequested) -> BankaccountProcessManagerState {
        let event = event.clone();
        BankaccountProcessManagerState {
            wire_transfer_id: event.wire_transfer_id,
            account_number: event.account_number,
            customer_id: event.customer_id,
            amount: event.amount,
            target_routing_number: event.target_routing_number,
            target_account_number: event.target_account_number,
            status: TransferStatus::Requested,
        }
    }
}