use crate::event::wrappers::Amount;

/// Only deposits and withdrawals count as operations.
/// All actions related to dispute is something different.
#[derive(Debug, PartialEq, Clone, Copy)]
pub(in crate::analysis) enum Kind {
    Deposit,
    Withdrawal,
}

/// Client's operation
#[derive(Debug, Clone, PartialEq)]
pub(in crate::analysis) struct Operation {
    pub kind: Kind,
    pub amount: Amount,
}
