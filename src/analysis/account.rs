use std::collections::HashMap;

use super::operation::Operation;
use crate::event::wrappers::{Amount, Tx};

#[derive(Debug, Default)]
pub(super) struct Account {
    pub available_amount: Amount,
    pub held_amount: Amount,
    pub operations: HashMap<Tx, Operation>,
}
