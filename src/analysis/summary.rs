use std::{collections::HashSet, iter::Iterator};

use super::{
    account::{self, Account},
    Analysis,
};
use crate::event::wrappers::Client;

#[derive(Debug)]
pub struct Summary {
    accounts: Vec<(Client, Account)>,
    locked: HashSet<Client>,
}

impl Iterator for Summary {
    type Item = account::Summary;

    fn next(&mut self) -> Option<Self::Item> {
        self.accounts.pop().map(|(client, account)| {
            let held = account.held_amount;
            let available = account.available_amount;

            account::Summary {
                client,
                available,
                held,
                total: (available + held).round(), // round amount after addition just in case
                locked: self.locked.contains(&client),
            }
        })
    }
}

impl From<Analysis> for Summary {
    fn from(analysis: Analysis) -> Self {
        let accounts = analysis.accounts.into_iter().collect();

        Self {
            accounts,
            locked: analysis.locked_accounts,
        }
    }
}
