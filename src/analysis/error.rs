use crate::event::wrappers::{Amount, Client, Tx};

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("negative amount operation (client: {0}, tx: {1}, amount: {2})")]
    NegativeAmountOperation(Client, Tx, Amount),
    #[error("duplicate operation (tx: {0})")]
    DuplicateOperation(Tx),
    #[error("account not found (client: {0})")]
    AccountNotFound(Client),
    #[error("insufficient funds (client: {0}, tx: {1})")]
    InsufficientFunds(Client, Tx, Amount),
    #[error("dispute already in progress (tx: {0})")]
    DisputeAlreadyInProgress(Tx),
    #[error("operation not found (client: {0}, tx: {1})")]
    OperationNotFound(Client, Tx),
    #[error("withdrawal dispute attempt (client: {0}, tx: {1})")]
    WithdrawalDisputeAttempt(Client, Tx),
    #[error("dispute not found (client: {0}, tx: {1})")]
    DisputeNotFound(Client, Tx),
    #[error("account locked (client: {0})")]
    AccountLocked(Client),
}
