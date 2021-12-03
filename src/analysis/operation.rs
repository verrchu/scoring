use crate::event::wrappers::Amount;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(super) enum Kind {
    Deposit,
    WithDrawal,
}

#[derive(Debug, PartialEq)]
pub(super) struct Operation {
    pub kind: Kind,
    pub amount: Amount,
    pub success: bool,
}
