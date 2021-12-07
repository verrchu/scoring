#[cfg(test)]
mod tests;

mod account;
pub use account::AccountSummary;
use account::{operation, Account, Operation};

/// provides [AnalysisSummary]
mod summary;
pub use summary::AnalysisSummary;

#[doc(hidden)]
mod error;
pub use error::Error as AnalysisError;

/// Utility error type which binds [AnalysisError] to [std::result::Result]
pub type AnalysisResult<T> = Result<T, AnalysisError>;

use std::collections::{hash_map::Entry, HashMap, HashSet};

use crate::event::wrappers::{Amount, Client, Tx};
use crate::event::Event;

/// Reprsents internal state of analysis process.
/// It's life cycle can be seen as follows:
/// # Example
/// ```
/// use scoring::{Analysis, Event};
///
/// let mut analysis = Analysis::begin();
///
/// let events = Vec::<Event>::new();
/// for event in events.iter() {
///     analysis.process_event(&event);
/// }
///
/// let symmary = analysis.summary();
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Analysis {
    #[doc(hidden)]
    // Tracks client accounts' states
    accounts: HashMap<Client, Account>,
    #[doc(hidden)]
    // Tracks active disputes
    disputes: HashMap<Tx, Client>,
    #[doc(hidden)]
    // Tracks locked accounts
    locked_accounts: HashSet<Client>,
    #[doc(hidden)]
    // Tracks used txs (transaction IDs). txs should be unique
    used_txs: HashSet<Tx>,
}

impl Analysis {
    /// Initializes analysis
    pub fn begin() -> Self {
        Analysis::default()
    }

    /// Turns [Analysis] into [AnalysisSummary]
    pub fn summary(self) -> AnalysisSummary {
        AnalysisSummary::from(self)
    }

    /// Processes single [event][Event]. Processing modifies [analysis][Analysis] internal state.
    pub fn process_event(&mut self, event: &Event) -> AnalysisResult<()> {
        match event {
            Event::Deposit { client, tx, amount } => self.process_deposit(*client, *tx, *amount),
            Event::Withdrawal { client, tx, amount } => {
                self.process_withdrawal(*client, *tx, *amount)
            }
            Event::Dispute { client, tx } => self.process_dispute_init(*client, *tx),
            Event::Resolve { client, tx } => self.process_dispute_resolve(*client, *tx),
            Event::Chargeback { client, tx } => self.process_dispute_chargeback(*client, *tx),
        }
    }

    #[doc(hidden)]
    fn process_deposit(&mut self, client: Client, tx: Tx, amount: Amount) -> AnalysisResult<()> {
        tracing::trace!(
            "attempting deposit: (client: {}, tx: {}, amount: {})",
            client,
            tx,
            amount
        );

        if self.locked_accounts.contains(&client) {
            return Err(AnalysisError::AccountLocked(client));
        }

        if self.used_txs.contains(&tx) {
            return Err(AnalysisError::DuplicateOperation(tx));
        }

        if amount.is_negative() {
            return Err(AnalysisError::NegativeAmountOperation(client, tx, amount));
        }

        let account = self.accounts.entry(client).or_default();

        match account.operations.entry(tx) {
            Entry::Vacant(entry) => {
                let operation = Operation {
                    kind: operation::Kind::Deposit,
                    amount,
                };

                tracing::trace!(
                    "deposit operation recorded: (client: {}, tx: {}, amount: {})",
                    client,
                    tx,
                    amount
                );

                entry.insert(operation);
            }
            // This branch shouldn't be reached
            Entry::Occupied(_) => return Err(AnalysisError::DuplicateOperation(tx)),
        }

        account.available_amount += amount;

        tracing::trace!(
            "available amount changed: (client: {}, amount: {}, delta: {})",
            client,
            account.available_amount,
            amount
        );

        self.used_txs.insert(tx);

        Ok(())
    }

    #[doc(hidden)]
    fn process_withdrawal(&mut self, client: Client, tx: Tx, amount: Amount) -> AnalysisResult<()> {
        tracing::trace!(
            "attempting withdrawal: (client: {}, tx: {}, amount: {})",
            client,
            tx,
            amount
        );

        if self.locked_accounts.contains(&client) {
            return Err(AnalysisError::AccountLocked(client));
        }

        if self.used_txs.contains(&tx) {
            return Err(AnalysisError::DuplicateOperation(tx));
        }

        if amount.is_negative() {
            return Err(AnalysisError::NegativeAmountOperation(client, tx, amount));
        }

        let mut account = match self.accounts.entry(client) {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(_) => return Err(AnalysisError::AccountNotFound(client)),
        };

        if amount > account.get().available_amount {
            return Err(AnalysisError::InsufficientFunds(client, tx, amount));
        }

        match account.get_mut().operations.entry(tx) {
            Entry::Vacant(entry) => {
                let operation = Operation {
                    kind: operation::Kind::Withdrawal,
                    amount,
                };

                tracing::trace!(
                    "withdrawal operation recorded: (client: {}, tx: {}, amount: {})",
                    client,
                    tx,
                    amount
                );

                entry.insert(operation);
            }
            // This branch shouldn't be reached
            Entry::Occupied(_) => return Err(AnalysisError::DuplicateOperation(tx)),
        }

        account.get_mut().available_amount -= amount;

        tracing::trace!(
            "available amount changed: (client: {}, amount: {}, delta: {})",
            client,
            account.get().available_amount,
            -amount
        );

        self.used_txs.insert(tx);

        Ok(())
    }

    #[doc(hidden)]
    fn process_dispute_init(&mut self, client: Client, tx: Tx) -> AnalysisResult<()> {
        tracing::trace!("attempting dispute init: (client: {}, tx: {})", client, tx);

        if self.locked_accounts.contains(&client) {
            return Err(AnalysisError::AccountLocked(client));
        }

        if self.disputes.contains_key(&tx) {
            return Err(AnalysisError::DisputeAlreadyInProgress(tx));
        }

        let mut account = match self.accounts.entry(client) {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(_) => return Err(AnalysisError::AccountNotFound(client)),
        };

        let amount = match account.get().operations.get(&tx) {
            Some(operation) => match operation.kind {
                operation::Kind::Deposit => operation.amount,
                operation::Kind::Withdrawal => {
                    return Err(AnalysisError::WithdrawalDisputeAttempt(client, tx))
                }
            },
            None => return Err(AnalysisError::OperationNotFound(client, tx)),
        };

        self.disputes.insert(tx, client);

        tracing::trace!("dispute inited: (client: {}, tx: {})", client, tx);

        account.get_mut().available_amount -= amount;

        tracing::trace!(
            "available amount changed: (client: {}, amount: {}, delta: {})",
            client,
            account.get().available_amount,
            -amount
        );

        account.get_mut().held_amount += amount;

        tracing::trace!(
            "held amount changed: (client: {}, amount: {}, delta: {})",
            client,
            account.get().held_amount,
            amount
        );

        Ok(())
    }

    #[doc(hidden)]
    fn process_dispute_resolve(&mut self, client: Client, tx: Tx) -> AnalysisResult<()> {
        tracing::trace!(
            "attempting dispute resolve: (client: {}, tx: {})",
            client,
            tx
        );

        if self.locked_accounts.contains(&client) {
            return Err(AnalysisError::AccountLocked(client));
        }

        match self.disputes.get(&tx) {
            Some(dispute_client) => {
                if *dispute_client != client {
                    return Err(AnalysisError::DisputeNotFound(client, tx));
                }
            }
            None => return Err(AnalysisError::DisputeNotFound(client, tx)),
        }

        let mut account = match self.accounts.entry(client) {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(_) => return Err(AnalysisError::AccountNotFound(client)),
        };

        let amount = match account.get().operations.get(&tx) {
            Some(operation) => operation.amount,
            // This branch shoulbn't be reached
            None => return Err(AnalysisError::OperationNotFound(client, tx)),
        };

        self.disputes.remove(&tx);

        tracing::trace!("dispute resolved: (client: {}, tx: {})", client, tx);

        account.get_mut().available_amount += amount;

        tracing::trace!(
            "available amount changed: (client: {}, amount: {}, delta: {})",
            client,
            account.get().available_amount,
            amount
        );

        account.get_mut().held_amount -= amount;

        tracing::trace!(
            "held amount changed: (client: {}, amount: {}, delta: {})",
            client,
            account.get().held_amount,
            -amount
        );

        Ok(())
    }

    #[doc(hidden)]
    fn process_dispute_chargeback(&mut self, client: Client, tx: Tx) -> AnalysisResult<()> {
        tracing::trace!(
            "attempting dispute chargeback: (client: {}, tx: {})",
            client,
            tx
        );

        if self.locked_accounts.contains(&client) {
            return Err(AnalysisError::AccountLocked(client));
        }

        match self.disputes.get(&tx) {
            Some(dispute_client) => {
                if *dispute_client != client {
                    return Err(AnalysisError::DisputeNotFound(client, tx));
                }
            }
            None => return Err(AnalysisError::DisputeNotFound(client, tx)),
        }

        let mut account = match self.accounts.entry(client) {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(_) => return Err(AnalysisError::AccountNotFound(client)),
        };

        let amount = match account.get().operations.get(&tx) {
            Some(operation) => operation.amount,
            None => return Err(AnalysisError::OperationNotFound(client, tx)),
        };

        self.disputes.remove(&tx);

        tracing::trace!("dispute charged back: (client: {}, tx: {})", client, tx);

        account.get_mut().held_amount -= amount;

        tracing::trace!(
            "held amount changed: (client: {}, amount: {}, delta: {})",
            client,
            account.get().held_amount,
            -amount
        );

        self.locked_accounts.insert(client);

        tracing::trace!("account locked: (client: {})", client,);

        Ok(())
    }
}
