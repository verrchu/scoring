use std::{collections::HashSet, iter::Iterator};

use super::{
    account::{Account, AccountSummary},
    Analysis,
};
use crate::event::wrappers::Client;

#[derive(Debug)]
/// Represents [Analysis] symmary and can be conveniently obtained
/// from an [Analysis] instance by calling [Analysis::summary]
///
/// Implements [Iterator] of [AccountSummary] for natural sequential processing
pub struct AnalysisSummary {
    #[doc(hidden)]
    accounts: Vec<(Client, Account)>,
    #[doc(hidden)]
    locked: HashSet<Client>,
}

impl Iterator for AnalysisSummary {
    type Item = AccountSummary;

    fn next(&mut self) -> Option<Self::Item> {
        self.accounts.pop().map(|(client, account)| {
            let held = account.held_amount;
            let available = account.available_amount;

            AccountSummary {
                client,
                available: available.to_string(),
                held: held.to_string(),
                total: (available + held).to_string(),
                locked: self.locked.contains(&client),
            }
        })
    }
}

impl From<Analysis> for AnalysisSummary {
    fn from(analysis: Analysis) -> Self {
        let accounts = analysis.accounts.into_iter().collect();

        Self {
            accounts,
            locked: analysis.locked_accounts,
        }
    }
}
