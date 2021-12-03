mod account;
use account::Account;
mod operation;
use operation::Operation;

mod error;
pub use error::Error as AnalysisError;

pub type AnalysisResult<T> = Result<T, AnalysisError>;

use std::collections::{hash_map::Entry, HashMap, HashSet};

use crate::event::wrappers::{Amount, Client, Tx};
use crate::event::Event;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Analysis {
    accounts: HashMap<Client, Account>,
    disputes: HashMap<Tx, Client>,
    locked_accounts: HashSet<Client>,
}

impl Analysis {
    pub fn begin() -> Self {
        Analysis::default()
    }

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
            Entry::Occupied(_) => return Err(AnalysisError::DuplicateOperation(tx)),
        }

        account.available_amount += amount;

        tracing::trace!(
            "available amount changed: (client: {}, amount: {}, delta: {})",
            client,
            account.available_amount,
            amount
        );

        Ok(())
    }

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
            Entry::Occupied(_) => return Err(AnalysisError::DuplicateOperation(tx)),
        }

        account.get_mut().available_amount -= amount;

        tracing::trace!(
            "available amount changed: (client: {}, amount: {}, delta: {})",
            client,
            account.get().available_amount,
            -amount
        );

        Ok(())
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_deposit_success() {
        let mut analysis = Analysis::begin();

        let client = Client(1);
        let tx = Tx(1);
        let amount = Amount(1.0);

        let event = Event::Deposit { client, tx, amount };

        let result = analysis.process_event(&event);
        assert_eq!(result, Ok(()));

        assert_eq!(analysis.accounts.len(), 1);
        assert!(analysis.locked_accounts.is_empty());
        assert!(analysis.disputes.is_empty());

        let account = analysis.accounts.get(&client);
        assert!(account.is_some());
        let account = account.unwrap();

        assert_eq!(account.available_amount, amount);
        assert_eq!(account.held_amount, Amount(0.0));
        assert_eq!(account.operations.len(), 1);

        let operation = account.operations.get(&tx);
        assert!(operation.is_some());
        let operation = operation.unwrap();

        assert_eq!(operation.amount, amount);
        assert_eq!(operation.kind, super::operation::Kind::Deposit);
    }

    #[test]
    fn test_deposit_failure_negative_amount() {
        let mut analysis = Analysis::begin();

        let client = Client(1);
        let tx = Tx(1);
        let amount = Amount(-1.0);

        let event = Event::Deposit { client, tx, amount };

        let result = analysis.process_event(&event);
        assert_eq!(
            result,
            Err(AnalysisError::NegativeAmountOperation(client, tx, amount))
        );

        assert!(analysis.accounts.is_empty());
        assert!(analysis.locked_accounts.is_empty());
        assert!(analysis.disputes.is_empty());
    }

    #[test]
    fn test_deposit_synthetic_failure_account_locked() {
        let mut analysis = Analysis::begin();

        let client = Client(1);
        let tx = Tx(1);
        let amount = Amount(-1.0);

        analysis.locked_accounts.insert(client);

        let event = Event::Deposit { client, tx, amount };

        let result = analysis.process_event(&event);
        assert_eq!(result, Err(AnalysisError::AccountLocked(client)));

        assert!(analysis.accounts.is_empty());
        assert!(analysis.disputes.is_empty());
        assert_eq!(
            analysis.locked_accounts,
            [client].into_iter().collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_deposit_failure_duplicate_operation() {
        let mut analysis = Analysis::begin();

        let client = Client(1);
        let tx = Tx(1);
        let amount = Amount(1.0);

        let event = Event::Deposit { client, tx, amount };

        let result = analysis.process_event(&event);
        assert_eq!(result, Ok(()));

        assert_eq!(
            analysis
                .accounts
                .keys()
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>(),
            vec![client]
        );

        let tmp_analysis = analysis.clone();

        let result = analysis.process_event(&event);
        assert_eq!(result, Err(AnalysisError::DuplicateOperation(tx)));

        // duplicate operation had no effect on analysis state
        assert_eq!(tmp_analysis, analysis);
    }
}
