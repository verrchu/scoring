use crate::event::wrappers::Amount;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(super) enum Kind {
    Deposit,
    Withdrawal,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct Operation {
    pub kind: Kind,
    pub amount: Amount,
}
