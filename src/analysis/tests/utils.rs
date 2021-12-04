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

pub(super) fn assert_operations_count(analysis: &Analysis, client: Client, count: usize) {
    let account = analysis.accounts.get(&client);
    assert!(account.is_some());
    let account = account.unwrap();

    assert_eq!(account.operations.len(), count);
}

pub(super) fn assert_disputes_count(analysis: &Analysis, count: usize) {
    assert_eq!(analysis.disputes.len(), count);
}

pub(super) fn assert_dispute_exists(analysis: &Analysis, expected_client: Client, tx: Tx) {
    let client = analysis.disputes.get(&tx);
    assert!(client.is_some());

    assert_eq!(*client.unwrap(), expected_client);
}

pub(super) fn assert_dispute_not_exists(analysis: &Analysis, expected_client: Client, tx: Tx) {
    let client = analysis.disputes.get(&tx);

    if let Some(client) = client {
        assert_ne!(*client, expected_client);
    }
}
