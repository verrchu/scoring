use crate::event::wrappers::{Amount, Client, Tx};

/// Represents business-errors which can occur during [Analysis][super::Analysis]
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    /// Attempt to [deposit][super::Event::Deposit] or
    /// [withdraw][super::Event::Withdrawal] negative [amount][Amount] \
    /// **Note that 0 amount for both types of transaction is allowed**
    #[error("negative amount operation (client: {0}, tx: {1}, amount: {2})")]
    NegativeAmountOperation(Client, Tx, Amount),

    /// [Transaction ID][Tx] occured more than once during [Analysis][super::Analysis]
    #[error("duplicate operation (tx: {0})")]
    DuplicateOperation(Tx),

    /// Specified [client ID][Client] not found
    #[error("account not found (client: {0})")]
    AccountNotFound(Client),

    /// Attempt to [withdraw][super::Event::Withdrawal] when transaction amount
    /// exceeds client's available funds
    #[error("insufficient funds (client: {0}, tx: {1})")]
    InsufficientFunds(Client, Tx, Amount),

    /// Attempt to [initiate dispute][super::Event::Dispute] on a
    /// transaction which is already under dispute
    #[error("dispute already in progress (tx: {0})")]
    DisputeAlreadyInProgress(Tx),

    /// [Transaction ID][Tx] is not found among [client's][Client] transaction
    #[error("operation not found (client: {0}, tx: {1})")]
    OperationNotFound(Client, Tx),

    /// Attempt to [initiate dispute][super::Event::Dispute] on a
    /// [withdrawal][super::Event::Withdrawal] transaction
    #[error("withdrawal dispute attempt (client: {0}, tx: {1})")]
    WithdrawalDisputeAttempt(Client, Tx),

    /// Dispute not found when attempting to [resolve][super::Event::Resolve] or
    /// [chargeback][super::Event::Chargeback]
    #[error("dispute not found (client: {0}, tx: {1})")]
    DisputeNotFound(Client, Tx),

    /// Operation can't be performed because account is locked
    #[error("account locked (client: {0})")]
    AccountLocked(Client),
}
