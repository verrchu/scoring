mod account;
use account::Account;
mod operation;
use operation::Operation;
mod error;
pub use error::Error as AnalysisError;

pub type AnalysisResult<T> = Result<T, AnalysisError>;

use std::collections::{hash_map::Entry, HashMap};

use crate::event::wrappers::{Amount, Client, Tx};
use crate::event::Event;

#[derive(Debug, Default)]
pub struct Analysis {
    accounts: HashMap<Client, Account>,
    disputes: HashMap<Tx, Client>,
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
            _ => todo!(),
        }
    }

    fn process_deposit(&mut self, client: Client, tx: Tx, amount: Amount) -> AnalysisResult<()> {
        if amount.is_negative() {
            return Err(AnalysisError::NegativeAmountDeposit(client, tx, amount));
        }

        let account = self.accounts.entry(client).or_default();
        account.available_amount += amount;

        match account.operations.entry(tx) {
            Entry::Vacant(entry) => {
                todo!();
                // entry.insert(amount);
                // Ok(())
            }
            Entry::Occupied(_) => todo!(),
        }
    }

    fn process_withdrawal(&mut self, client: Client, tx: Tx, amount: Amount) -> AnalysisResult<()> {
        let account = self.accounts.entry(client).or_default();
        account.available_amount -= amount;

        match account.operations.entry(tx) {
            Entry::Vacant(entry) => {
                todo!();
                // entry.insert(amount);
                // Ok(())
            }
            Entry::Occupied(_) => todo!(),
        }
    }
}
