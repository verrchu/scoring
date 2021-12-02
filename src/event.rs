use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Event {
    Chargeback { client: u64, tx: u64, amount: f64 },
    Deposit { client: u64, tx: u64, amount: f64 },
    Dispute { client: u64, tx: u64 },
    Resolve { client: u64, tx: u64 },
    Withdrawal { client: u64, tx: u64 },
}
