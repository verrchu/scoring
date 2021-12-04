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
    use super::operation::Kind as OperationKind;
    use super::*;

    use pretty_assertions::assert_eq;

    mod utils {
        use super::*;

        use pretty_assertions::assert_eq;

        pub(super) fn assert_operation_exists(
            analysis: &Analysis,
            client: Client,
            tx: Tx,
            expected_operation: Operation,
        ) {
            let account = analysis.accounts.get(&client);
            assert!(account.is_some());
            let account = account.unwrap();

            let operation = account.operations.get(&tx);
            assert!(operation.is_some());
            let operation = operation.unwrap();

            assert_eq!(expected_operation, operation.to_owned());
        }

        pub(super) fn assert_account_exists(analysis: &Analysis, client: Client) {
            let account = analysis.accounts.get(&client);
            assert!(account.is_some());
        }

        pub(super) fn assert_account_not_exists(analysis: &Analysis, client: Client) {
            let account = analysis.accounts.get(&client);
            assert!(account.is_none());
        }

        pub(super) fn assert_account_balance(
            analysis: &Analysis,
            client: Client,
            available_amount: Amount,
            held_amount: Amount,
        ) {
            let account = analysis.accounts.get(&client);
            assert!(account.is_some());
            let account = account.unwrap();

            assert_eq!(account.available_amount, available_amount);
            assert_eq!(account.held_amount, held_amount);
        }

        pub(super) fn assert_account_locked(analysis: &Analysis, client: Client) {
            assert!(analysis.locked_accounts.contains(&client));
        }

        pub(super) fn assert_operation_count(analysis: &Analysis, client: Client, count: usize) {
            let account = analysis.accounts.get(&client);
            assert!(account.is_some());
            let account = account.unwrap();

            assert_eq!(account.operations.len(), count);
        }
    }

    #[test]
    fn test_deposit_success() {
        let mut analysis = Analysis::begin();

        let client = Client(1);
        let tx = Tx(1);
        let amount = Amount(1.0);

        utils::assert_account_not_exists(&analysis, client);

        let event = Event::Deposit { client, tx, amount };

        let result = analysis.process_event(&event);
        assert_eq!(result, Ok(()));

        utils::assert_account_exists(&analysis, client);

        utils::assert_operation_exists(
            &analysis,
            client,
            tx,
            Operation {
                kind: OperationKind::Deposit,
                amount,
            },
        );

        utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));
        utils::assert_account_exists(&analysis, client);
    }

    #[test]
    fn test_deposit_failure_negative_amount_operation() {
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

        utils::assert_account_not_exists(&analysis, client);
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

        // This is a synthetic test. Therefore this counter-intuitive state
        // where account is locked but does not exist is expected
        utils::assert_account_not_exists(&analysis, client);
        utils::assert_account_locked(&analysis, client);
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

        utils::assert_operation_exists(
            &analysis,
            client,
            tx,
            Operation {
                kind: OperationKind::Deposit,
                amount,
            },
        );
        utils::assert_operation_count(&analysis, client, 1);
        utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));

        let result = analysis.process_event(&event);
        assert_eq!(result, Err(AnalysisError::DuplicateOperation(tx)));

        utils::assert_operation_count(&analysis, client, 1);
        utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));
    }

    #[test]
    fn test_withdrawal_success() {
        let mut analysis = Analysis::begin();

        let client = Client(1);
        let tx = Tx(1);
        let amount = Amount(1.0);

        utils::assert_account_not_exists(&analysis, client);

        let event = Event::Deposit { client, tx, amount };

        let result = analysis.process_event(&event);
        assert_eq!(result, Ok(()));

        utils::assert_operation_exists(
            &analysis,
            client,
            tx,
            Operation {
                kind: OperationKind::Deposit,
                amount,
            },
        );
        utils::assert_operation_count(&analysis, client, 1);
        utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));

        let client = Client(1);
        let tx = Tx(2);
        let amount = Amount(1.0);

        utils::assert_account_exists(&analysis, client);

        let event = Event::Withdrawal { client, tx, amount };

        let result = analysis.process_event(&event);
        assert_eq!(result, Ok(()));

        utils::assert_operation_exists(
            &analysis,
            client,
            tx,
            Operation {
                kind: OperationKind::Withdrawal,
                amount,
            },
        );
        utils::assert_operation_count(&analysis, client, 2);
        utils::assert_account_balance(&analysis, client, Amount(0.0), Amount(0.0));
    }

    #[test]
    fn test_withdrawal_failure_account_not_found() {
        let mut analysis = Analysis::begin();

        let client = Client(1);
        let tx = Tx(2);
        let amount = Amount(1.0);

        utils::assert_account_not_exists(&analysis, client);

        let event = Event::Withdrawal { client, tx, amount };

        let result = analysis.process_event(&event);
        assert_eq!(result, Err(AnalysisError::AccountNotFound(client)));

        utils::assert_account_not_exists(&analysis, client);
    }

    #[test]
    fn test_withdrawal_failure_negative_amount_operations() {
        let mut analysis = Analysis::begin();

        let client = Client(1);

        let tx = Tx(2);
        let amount = Amount(1.0);

        let event = Event::Deposit { client, tx, amount };

        let result = analysis.process_event(&event);
        assert_eq!(result, Ok(()));

        utils::assert_operation_count(&analysis, client, 1);
        utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));

        let tx = Tx(2);
        let amount = Amount(-1.0);

        let event = Event::Withdrawal { client, tx, amount };

        let result = analysis.process_event(&event);
        assert_eq!(
            result,
            Err(AnalysisError::NegativeAmountOperation(client, tx, amount))
        );

        utils::assert_operation_count(&analysis, client, 1);
        utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));
    }

    #[test]
    fn test_withdrawal_failure_insufficient_funds() {
        let mut analysis = Analysis::begin();

        let client = Client(1);

        let tx = Tx(2);
        let amount = Amount(1.0);

        let event = Event::Deposit { client, tx, amount };

        let result = analysis.process_event(&event);
        assert_eq!(result, Ok(()));

        utils::assert_operation_count(&analysis, client, 1);
        utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));

        let tx = Tx(2);
        let amount = Amount(1.5);

        let event = Event::Withdrawal { client, tx, amount };

        let result = analysis.process_event(&event);
        assert_eq!(
            result,
            Err(AnalysisError::InsufficientFunds(client, tx, amount))
        );

        utils::assert_operation_count(&analysis, client, 1);
        utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));
    }

    #[test]
    fn test_withdrawal_synthetic_failure_account_locked() {
        let mut analysis = Analysis::begin();

        let client = Client(1);
        let tx = Tx(1);
        let amount = Amount(-1.0);

        analysis.locked_accounts.insert(client);

        let event = Event::Withdrawal { client, tx, amount };

        let result = analysis.process_event(&event);
        assert_eq!(result, Err(AnalysisError::AccountLocked(client)));

        // This is a synthetic test. Therefore this counter-intuitive state
        // where account is locked but does not exist is expected
        utils::assert_account_not_exists(&analysis, client);
        utils::assert_account_locked(&analysis, client);
    }

    #[test]
    fn test_withdrawal_failure_duplicate_operation() {
        let mut analysis = Analysis::begin();

        let client = Client(1);
        let tx = Tx(1);
        let amount = Amount(1.0);

        let event = Event::Deposit { client, tx, amount };

        let result = analysis.process_event(&event);
        assert_eq!(result, Ok(()));

        utils::assert_operation_exists(
            &analysis,
            client,
            tx,
            Operation {
                kind: OperationKind::Deposit,
                amount,
            },
        );
        utils::assert_operation_count(&analysis, client, 1);
        utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));

        let event = Event::Withdrawal { client, tx, amount };

        let result = analysis.process_event(&event);
        assert_eq!(result, Err(AnalysisError::DuplicateOperation(tx)));

        utils::assert_operation_count(&analysis, client, 1);
        utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));
    }
}
