use super::operation::Kind as OperationKind;
use super::*;

#[test]
fn test_success() {
    let mut analysis = Analysis::begin();

    let client = Client(1);
    let tx = Tx(1);
    let amount = Amount(1.0);

    utils::assert_account_not_exists(&analysis, client);

    // A deposit with sufficient amount should occur before a withdrawal
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
    utils::assert_operations_count(&analysis, client, 1);
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
    utils::assert_operations_count(&analysis, client, 2);
    utils::assert_account_balance(&analysis, client, Amount(0.0), Amount(0.0));
}

#[test]
fn test_failure_account_not_found() {
    let mut analysis = Analysis::begin();

    let client = Client(1);
    let tx = Tx(2);
    let amount = Amount(1.0);

    utils::assert_account_not_exists(&analysis, client);

    // Withdrawal can happen only if account record has previously been created
    let event = Event::Withdrawal { client, tx, amount };

    let result = analysis.process_event(&event);
    assert_eq!(result, Err(AnalysisError::AccountNotFound(client)));

    utils::assert_account_not_exists(&analysis, client);
}

#[test]
fn test_failure_negative_amount_operations() {
    let mut analysis = Analysis::begin();

    let client = Client(1);

    let tx = Tx(1);
    let amount = Amount(1.0);

    let event = Event::Deposit { client, tx, amount };

    let result = analysis.process_event(&event);
    assert_eq!(result, Ok(()));

    utils::assert_operations_count(&analysis, client, 1);
    utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));

    let tx = Tx(2);
    let amount = Amount(-1.0);

    let event = Event::Withdrawal { client, tx, amount };

    let result = analysis.process_event(&event);
    assert_eq!(
        result,
        Err(AnalysisError::NegativeAmountOperation(client, tx, amount))
    );

    utils::assert_operations_count(&analysis, client, 1);
    utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));
}

#[test]
fn test_failure_insufficient_funds() {
    let mut analysis = Analysis::begin();

    let client = Client(1);

    let tx = Tx(1);
    let amount = Amount(1.0);

    let event = Event::Deposit { client, tx, amount };

    let result = analysis.process_event(&event);
    assert_eq!(result, Ok(()));

    utils::assert_operations_count(&analysis, client, 1);
    utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));

    let tx = Tx(2);
    let amount = Amount(1.5);

    let event = Event::Withdrawal { client, tx, amount };

    let result = analysis.process_event(&event);
    assert_eq!(
        result,
        Err(AnalysisError::InsufficientFunds(client, tx, amount))
    );

    utils::assert_operations_count(&analysis, client, 1);
    utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));
}

#[test]
fn test_synthetic_failure_account_locked() {
    let mut analysis = Analysis::begin();

    let client = Client(1);
    let tx = Tx(1);
    let amount = Amount(-1.0);

    // Lock account
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
fn test_failure_duplicate_operations_same_account() {
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
    utils::assert_operations_count(&analysis, client, 1);
    utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));

    // Emit event with same tx
    let event = Event::Withdrawal { client, tx, amount };

    let result = analysis.process_event(&event);
    assert_eq!(result, Err(AnalysisError::DuplicateOperation(tx)));

    utils::assert_operations_count(&analysis, client, 1);
    utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));
}

#[test]
fn test_failure_duplicate_operations_different_accounts() {
    let mut analysis = Analysis::begin();

    let client = Client(1);
    let tx = Tx(1);
    let amount = Amount(1.0);

    let event = Event::Deposit { client, tx, amount };

    let result = analysis.process_event(&event);
    assert_eq!(result, Ok(()));

    utils::assert_operations_count(&analysis, client, 1);
    utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));

    let client = Client(2);
    let tx = Tx(2);
    let amount = Amount(1.0);

    let event = Event::Deposit { client, tx, amount };

    let result = analysis.process_event(&event);
    assert_eq!(result, Ok(()));

    utils::assert_operations_count(&analysis, client, 1);
    utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));

    let client = Client(2);
    let tx = Tx(1);
    let amount = Amount(1.0);

    let event = Event::Deposit { client, tx, amount };

    let result = analysis.process_event(&event);
    assert_eq!(result, Err(AnalysisError::DuplicateOperation(tx)));

    utils::assert_operations_count(&analysis, client, 1);
    utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));
}
