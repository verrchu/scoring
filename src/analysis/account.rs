use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::operation::Operation;
use crate::event::wrappers::{Amount, Client, Tx};

#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct Account {
    pub(super) available_amount: Amount,
    pub(super) held_amount: Amount,
    pub(super) operations: HashMap<Tx, Operation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub(super) client: Client,
    pub(super) available: Amount,
    pub(super) held: Amount,
    pub(super) total: Amount,
    pub(super) locked: bool,
}
