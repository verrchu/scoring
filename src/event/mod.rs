pub mod raw;
pub mod wrappers;
pub use raw::RawEvent;

use wrappers::{Amount, Client, Tx};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Event {
    Chargeback {
        client: Client,
        tx: Tx,
    },
    Deposit {
        client: Client,
        tx: Tx,
        amount: Amount,
    },
    Dispute {
        client: Client,
        tx: Tx,
    },
    Resolve {
        client: Client,
        tx: Tx,
    },
    Withdrawal {
        client: Client,
        tx: Tx,
        amount: Amount,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Chargeback,
    Deposit,
    Dispute,
    Resolve,
    Withdrawal,
}
