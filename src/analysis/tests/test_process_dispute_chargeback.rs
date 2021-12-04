use super::*;

#[test]
fn test_success() {
    let mut analysis = Analysis::begin();

    let client = Client(1);

    let tx = Tx(1);
    let amount = Amount(1.0);

    let event = Event::Deposit { client, tx, amount };

    let result = analysis.process_event(&event);
    assert_eq!(result, Ok(()));

    let tx = Tx(2);
    let amount = Amount(2.0);

    let event = Event::Deposit { client, tx, amount };

    let result = analysis.process_event(&event);
    assert_eq!(result, Ok(()));

    utils::assert_account_balance(&analysis, client, Amount(3.0), Amount(0.0));
    utils::assert_operations_count(&analysis, client, 2);

    let event = Event::Dispute { client, tx: Tx(1) };

    let result = analysis.process_event(&event);
    assert_eq!(result, Ok(()));

    utils::assert_account_balance(&analysis, client, Amount(2.0), Amount(1.0));
    utils::assert_operations_count(&analysis, client, 2);

    utils::assert_disputes_count(&analysis, 1);
    utils::assert_dispute_exists(&analysis, client, Tx(1));

    let event = Event::Chargeback { client, tx: Tx(1) };

    let result = analysis.process_event(&event);
    assert_eq!(result, Ok(()));

    utils::assert_account_balance(&analysis, client, Amount(2.0), Amount(0.0));
    utils::assert_operations_count(&analysis, client, 2);

    utils::assert_disputes_count(&analysis, 0);
    utils::assert_account_locked(&analysis, client);
}

#[test]
fn test_failure_dipute_not_found_by_tx() {
    let mut analysis = Analysis::begin();

    let client = Client(1);

    let tx = Tx(1);
    let amount = Amount(1.0);

    let event = Event::Deposit { client, tx, amount };

    let result = analysis.process_event(&event);
    assert_eq!(result, Ok(()));

    let event = Event::Dispute { client, tx };

    let result = analysis.process_event(&event);
    assert_eq!(result, Ok(()));

    utils::assert_account_balance(&analysis, client, Amount(0.0), Amount(1.0));
    utils::assert_operations_count(&analysis, client, 1);

    utils::assert_disputes_count(&analysis, 1);
    utils::assert_dispute_exists(&analysis, client, Tx(1));

    let event = Event::Chargeback { client, tx: Tx(2) };

    let result = analysis.process_event(&event);
    assert_eq!(result, Err(AnalysisError::DisputeNotFound(client, Tx(2))));

    utils::assert_account_balance(&analysis, client, Amount(0.0), Amount(1.0));
    utils::assert_operations_count(&analysis, client, 1);

    utils::assert_disputes_count(&analysis, 1);
    utils::assert_dispute_exists(&analysis, client, Tx(1));
}

#[test]
fn test_failure_dipute_not_found_by_client() {
    let mut analysis = Analysis::begin();

    let client = Client(1);

    let tx = Tx(1);
    let amount = Amount(1.0);

    let event = Event::Deposit { client, tx, amount };

    let result = analysis.process_event(&event);
    assert_eq!(result, Ok(()));

    let event = Event::Dispute { client, tx };

    let result = analysis.process_event(&event);
    assert_eq!(result, Ok(()));

    utils::assert_account_balance(&analysis, client, Amount(0.0), Amount(1.0));
    utils::assert_operations_count(&analysis, client, 1);

    utils::assert_disputes_count(&analysis, 1);
    utils::assert_dispute_exists(&analysis, client, Tx(1));

    let event = Event::Chargeback {
        client: Client(2),
        tx,
    };

    let result = analysis.process_event(&event);
    assert_eq!(result, Err(AnalysisError::DisputeNotFound(Client(2), tx)));

    utils::assert_account_balance(&analysis, client, Amount(0.0), Amount(1.0));
    utils::assert_operations_count(&analysis, client, 1);

    utils::assert_disputes_count(&analysis, 1);
    utils::assert_dispute_exists(&analysis, client, Tx(1));
}

#[test]
fn test_synthetic_failure_account_not_found() {
    let mut analysis = Analysis::begin();

    let client = Client(1);
    let tx = Tx(1);

    assert!(analysis.disputes.insert(tx, client).is_none());

    utils::assert_disputes_count(&analysis, 1);
    utils::assert_dispute_exists(&analysis, client, tx);

    let event = Event::Chargeback { client, tx };

    utils::assert_account_not_exists(&analysis, client);

    let result = analysis.process_event(&event);
    assert_eq!(result, Err(AnalysisError::AccountNotFound(client)));

    utils::assert_account_not_exists(&analysis, client);

    utils::assert_disputes_count(&analysis, 1);
    utils::assert_dispute_exists(&analysis, client, tx);
}

#[test]
fn test_synthetic_failure_operation_not_found() {
    let mut analysis = Analysis::begin();

    let client = Client(1);

    let tx = Tx(1);
    let amount = Amount(1.0);

    let event = Event::Deposit { client, tx, amount };

    let result = analysis.process_event(&event);
    assert_eq!(result, Ok(()));

    utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));
    utils::assert_operations_count(&analysis, client, 1);

    assert!(analysis.disputes.insert(Tx(2), client).is_none());

    utils::assert_disputes_count(&analysis, 1);
    utils::assert_dispute_exists(&analysis, client, Tx(2));

    let event = Event::Chargeback { client, tx: Tx(2) };

    let result = analysis.process_event(&event);
    assert_eq!(result, Err(AnalysisError::OperationNotFound(client, Tx(2))));

    utils::assert_account_balance(&analysis, client, Amount(1.0), Amount(0.0));
    utils::assert_operations_count(&analysis, client, 1);

    utils::assert_disputes_count(&analysis, 1);
    utils::assert_dispute_exists(&analysis, client, Tx(2));
}

#[test]
fn test_synthetic_failure_account_locked() {
    let mut analysis = Analysis::begin();

    let client = Client(1);
    let tx = Tx(1);

    // Lock account
    analysis.locked_accounts.insert(client);

    let event = Event::Chargeback { client, tx };

    let result = analysis.process_event(&event);
    assert_eq!(result, Err(AnalysisError::AccountLocked(client)));

    // This is a synthetic test. Therefore this counter-intuitive state
    // where account is locked but does not exist is expected
    utils::assert_account_not_exists(&analysis, client);
    utils::assert_account_locked(&analysis, client);
    utils::assert_disputes_count(&analysis, 0);
}
