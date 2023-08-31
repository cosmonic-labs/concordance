use std::collections::HashMap;

use crate::*;

use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::{GetResponse, KeyValue, KeyValueSender, SetRequest};
use wasmcloud_interface_logging::{debug, error};

// Note an invariant: the last() element in a ledger's effective_balance field is
// always the same as the balance stored in the balance.{account} key.

/// Creates a new AccountLedger instance with an initial transaction as a deposit,
/// sets the current balance to the initial amount
pub async fn initialize_account(event: AccountCreated) -> Result<()> {
    debug!("Initializing account {}", event.account_number);
    let kv = KeyValueSender::new();

    let account_number = event.account_number.to_string();
    let ctx = Context::default();

    let initial_balance = event.initial_balance.unwrap_or_default() as u32;

    // Set up the initial ledger
    let ledger_key = format!("ledger.{account_number}");
    let ledger = AccountLedger::new(event.account_number, initial_balance);
    let ledger_json = serde_json::to_string(&ledger).unwrap(); // we know this won't fail

    // set the current balance
    let balance_key = format!("balance.{account_number}");

    set(&ctx, &kv, ledger_key, ledger_json).await;
    set(&ctx, &kv, balance_key, initial_balance.to_string()).await;

    Ok(())
}

/// Records a deposit by adding a `LedgerLine` to the end of the previously stored
/// ledger and recording the new balance.
pub async fn record_funds_deposited(event: FundsDeposited) -> Result<()> {
    debug!("Recording deposit in account {}", event.account_number);
    let account_number = event.account_number.to_string();
    let ctx = Context::default();

    let kv = KeyValueSender::new();
    let ledger_key = format!("ledger.{account_number}");

    let new_ledger = get(&ctx, &kv, &ledger_key).await.map(|ledger_raw| {
        serde_json::from_str::<AccountLedger>(&ledger_raw).map(|mut ledger| {
            let last_balance = ledger.ledger_lines.last().unwrap().effective_balance;
            ledger.ledger_lines.push(LedgerLine {
                amount: event.amount as u32,
                tx_type: TransactionType::Deposit,
                effective_balance: last_balance + event.amount as u32,
            });
            ledger
        })
    });
    if let Some(Ok(ledger)) = new_ledger {
        let new_balance = ledger
            .ledger_lines
            .last()
            .map(|l| l.effective_balance)
            .unwrap_or(0);
        set_ledger(&ctx, &kv, ledger_key, ledger).await;
        let balance_key = format!("balance.{account_number}");
        set(&ctx, &kv, balance_key, new_balance.to_string()).await;
    } else {
        error!("Unable to save projection for deposit on account {account_number}");
    }

    Ok(())
}

/// Records a reservation of funds by adding a funds reserved transaction to the end of the
/// ledger and recording the newly adjusted balance
pub async fn record_funds_reserved(event: FundsReserved) -> Result<()> {
    debug!(
        "Recording funds reservation (interbank) in account {}",
        event.account_number
    );
    let account_number = event.account_number.to_string();
    let ctx = Context::default();

    let kv = KeyValueSender::new();
    let ledger_key = format!("ledger.{account_number}");

    let new_ledger = get(&ctx, &kv, &ledger_key).await.map(|ledger_raw| {
        serde_json::from_str::<AccountLedger>(&ledger_raw).map(|mut ledger| {
            let last_balance = ledger.ledger_lines.last().unwrap().effective_balance;
            ledger
                .holds
                .insert(event.wire_transfer_id, event.amount as u32);
            ledger.ledger_lines.push(LedgerLine {
                amount: event.amount as u32,
                tx_type: TransactionType::FundsReserve,
                effective_balance: last_balance - event.amount as u32,
            });
            ledger
        })
    });
    if let Some(Ok(ledger)) = new_ledger {
        let new_balance = ledger
            .ledger_lines
            .last()
            .map(|l| l.effective_balance)
            .unwrap_or(0);
        set_ledger(&ctx, &kv, ledger_key, ledger).await;
        let balance_key = format!("balance.{account_number}");
        set(&ctx, &kv, balance_key, new_balance.to_string()).await;
    } else {
        error!("Unable to save projection for withdrawal on account {account_number}");
    }

    Ok(())
}

// Releases previously reserved funds by adding a funds released transaction to the end
/// of the ledger and recording the updated balance
pub async fn record_funds_released(event: FundsReleased) -> Result<()> {
    debug!(
        "Recording funds release (interbank) in account {}",
        event.account_number
    );
    let account_number = event.account_number.to_string();

    let kv = KeyValueSender::new();
    let ledger_key = format!("ledger.{account_number}");
    let ctx = Context::default();

    let new_ledger = get(&ctx, &kv, &ledger_key).await.map(|ledger_raw| {
        serde_json::from_str::<AccountLedger>(&ledger_raw).map(|mut ledger| {
            let last_balance = ledger.ledger_lines.last().unwrap().effective_balance;
            let orig_hold = ledger.holds.remove(&event.wire_transfer_id);
            ledger.ledger_lines.push(LedgerLine {
                amount: orig_hold.unwrap_or_default(),
                tx_type: TransactionType::FundsRelease,
                effective_balance: last_balance + orig_hold.unwrap_or_default(),
            });
            ledger
        })
    });
    if let Some(Ok(ledger)) = new_ledger {
        let new_balance = ledger
            .ledger_lines
            .last()
            .map(|l| l.effective_balance)
            .unwrap_or(0);
        set_ledger(&ctx, &kv, ledger_key, ledger).await;
        let balance_key = format!("balance.{account_number}");
        set(&ctx, &kv, balance_key, new_balance.to_string()).await;
    } else {
        error!("Unable to save projection for withdrawal on account {account_number}");
    }

    Ok(())
}

/// Records a withdrawal from an account by adding a withdrawal ledger item to the
/// ledger and recording the new balance
pub async fn record_funds_withdrawn(event: FundsWithdrawn) -> Result<()> {
    debug!("Recording withdrawal in account {}", event.account_number);
    let account_number = event.account_number.to_string();

    let kv = KeyValueSender::new();
    let ledger_key = format!("ledger.{account_number}");

    let ctx = Context::default();

    // Note:the aggregate would prevent the creation of an event that would violate
    // business rules, so we can safely do the subtraction here without any guards

    let new_ledger = get(&ctx, &kv, &ledger_key).await.map(|ledger_raw| {
        serde_json::from_str::<AccountLedger>(&ledger_raw).map(|mut ledger| {
            let last_balance = ledger.ledger_lines.last().unwrap().effective_balance;
            ledger.ledger_lines.push(LedgerLine {
                amount: event.amount as u32,
                tx_type: TransactionType::Withdrawal,
                effective_balance: last_balance - event.amount as u32,
            });
            ledger
        })
    });
    if let Some(Ok(ledger)) = new_ledger {
        let new_balance = ledger
            .ledger_lines
            .last()
            .map(|l| l.effective_balance)
            .unwrap_or(0);
        set_ledger(&ctx, &kv, ledger_key, ledger).await;
        let balance_key = format!("balance.{account_number}");
        set(&ctx, &kv, balance_key, new_balance.to_string()).await;
    } else {
        error!("Unable to save projection for withdrawal on account {account_number}");
    }

    Ok(())
}

async fn set(ctx: &Context, kv: &KeyValueSender<WasmHost>, key: String, value: String) {
    if let Err(e) = kv
        .set(
            ctx,
            &SetRequest {
                key: key.clone(),
                value,
                expires: 0,
            },
        )
        .await
    {
        error!("Failed to set {key} in store: {e}");
    }
}

async fn set_ledger(
    ctx: &Context,
    kv: &KeyValueSender<WasmHost>,
    key: String,
    ledger: AccountLedger,
) {
    set(ctx, kv, key, serde_json::to_string(&ledger).unwrap()).await
}

async fn get(ctx: &Context, kv: &KeyValueSender<WasmHost>, key: &str) -> Option<String> {
    match kv.get(ctx, key).await {
        Ok(GetResponse {
            value: v,
            exists: true,
        }) => Some(v),
        Ok(GetResponse { exists: false, .. }) => None,
        Err(e) => {
            error!("Failed to get {key} from store: {e}");
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AccountLedger {
    pub account_number: String,
    pub ledger_lines: Vec<LedgerLine>,
    pub holds: HashMap<String, u32>,
}

impl AccountLedger {
    fn new(account_number: String, initial_balance: u32) -> AccountLedger {
        AccountLedger {
            account_number,
            holds: HashMap::new(),
            ledger_lines: vec![LedgerLine {
                amount: initial_balance,
                tx_type: TransactionType::Deposit,
                effective_balance: initial_balance,
            }],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LedgerLine {
    pub amount: u32,
    pub tx_type: TransactionType,
    pub effective_balance: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
enum TransactionType {
    Withdrawal,
    Deposit,
    Transfer,
    FundsReserve,
    FundsRelease,
    Unknown,
}
