use super::operation::Kind as OperationKind;
use super::*;

#[test]
fn test_success() {
    let mut analysis = Analysis::begin();

    let client = Client(1);
    let tx = Tx(1);
    let amount = Amount(1.0);

    utils::assert_account_not_exists(&analysis, client);

    let event = Event::Deposit { client, tx, amount };

    let result = analysis.process_event(&event);
    assert_eq!(result, Ok(()));

    // Account record should appear automatically on first deposit
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
fn test_failure_negative_amount_operation() {
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
fn test_synthetic_failure_account_locked() {
    let mut analysis = Analysis::begin();

    let client = Client(1);
    let tx = Tx(1);
    let amount = Amount(-1.0);

    // Lock account
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
fn test_failure_duplicate_operation() {
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

    // Repeat the same operation
    let result = analysis.process_event(&event);
    assert_eq!(result, Err(AnalysisError::DuplicateOperation(tx)));

    utils::assert_operations_count(&analysis, client, 1);
    utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));
}
