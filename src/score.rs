use std::collections::HashMap;

use crate::event::wrappers::{Amount, Client, Tx};
use crate::event::Event;

#[derive(Debug, Default)]
pub struct Score {
    accounts: HashMap<Client, Account>,
}

#[derive(Debug, Default)]
struct Account {
    available_amount: Amount,
    held_amount: Amount,
    txs: HashMap<Tx, Amount>,
}

impl Score {
    pub fn new() -> Self {
        Score::default()
    }

    pub fn process_event(&mut self, event: &Event) -> eyre::Result<()> {
        match event {
            Event::Deposit { client, tx, amount } => self.process_deposit(*client, *tx, *amount),
            Event::Withdrawal { client, tx, amount } => {
                self.process_withdrawal(*client, *tx, *amount)
            }
            _ => todo!(),
        }
    }

    fn process_deposit(&mut self, client: Client, tx: Tx, amount: Amount) -> eyre::Result<()> {
        let account = self.accounts.entry(client).or_default();
        account.available_amount += amount;

        account
            .txs
            .insert(tx, amount)
            .ok_or_else(|| eyre::eyre!("Tx {} already encountered", tx.0))
            .map(|_| ())
    }

    fn process_withdrawal(&mut self, client: Client, tx: Tx, amount: Amount) -> eyre::Result<()> {
        let account = self.accounts.entry(client).or_default();
        account.available_amount -= amount;

        account
            .txs
            .insert(tx, amount)
            .ok_or_else(|| eyre::eyre!("Tx {} already encountered", tx.0))
            .map(|_| ())
    }
}
