use crate::event::wrappers::{Amount, Client, Tx};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("negative amount operation (tx: {0}, amount: {1})")]
    NegativeAmountOperation(Tx, Amount),
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
}
