/// provides utility struct [RawEvent]
pub mod raw;
/// provides wrappers for primitives used in [Event]
pub mod wrappers;
pub use raw::RawEvent;

use wrappers::{Amount, Client, Tx};

use serde::{Deserialize, Serialize};

/// Represents all possible interactions of a client with the payment system
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Event {
    /// Chargeback as a result of a dispute
    Chargeback { client: Client, tx: Tx },
    /// Deposit transaction
    Deposit {
        client: Client,
        tx: Tx,
        amount: Amount,
    },
    /// Dispute init
    Dispute { client: Client, tx: Tx },
    /// Dispute resolve
    Resolve { client: Client, tx: Tx },
    /// Deposit transaction
    Withdrawal {
        client: Client,
        tx: Tx,
        amount: Amount,
    },
}

/// Utility list of all [event][Event] types
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Chargeback,
    Deposit,
    Dispute,
    Resolve,
    Withdrawal,
}
