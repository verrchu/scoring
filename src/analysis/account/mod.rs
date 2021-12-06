mod summary;
pub use summary::Summary as AccountSummary;
pub(super) mod operation;
pub(super) use operation::Operation;

use std::collections::HashMap;

use crate::event::wrappers::{Amount, Tx};

/// Represents client's account state during analysis
#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct Account {
    pub(super) available_amount: Amount,
    pub(super) held_amount: Amount,
    /// Tracks client's operations
    pub(super) operations: HashMap<Tx, Operation>,
}
