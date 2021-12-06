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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::{
        event::wrappers::{Amount, Tx},
        AccountSummary, Event,
    };

    use pretty_assertions::assert_eq;

    #[test]
    fn test_basic() {
        let mut analysis = Analysis::begin();

        analysis
            .process_event(&Event::Deposit {
                client: Client(1),
                tx: Tx(1),
                amount: Amount(1.0),
            })
            .unwrap();
        analysis
            .process_event(&Event::Deposit {
                client: Client(1),
                tx: Tx(2),
                amount: Amount(1.0),
            })
            .unwrap();
        analysis
            .process_event(&Event::Dispute {
                client: Client(1),
                tx: Tx(1),
            })
            .unwrap();

        analysis
            .process_event(&Event::Deposit {
                client: Client(2),
                tx: Tx(3),
                amount: Amount(10.0),
            })
            .unwrap();
        analysis
            .process_event(&Event::Withdrawal {
                client: Client(2),
                tx: Tx(4),
                amount: Amount(1.0),
            })
            .unwrap();
        analysis
            .process_event(&Event::Dispute {
                client: Client(2),
                tx: Tx(3),
            })
            .unwrap();
        analysis
            .process_event(&Event::Resolve {
                client: Client(2),
                tx: Tx(3),
            })
            .unwrap();

        analysis
            .process_event(&Event::Deposit {
                client: Client(3),
                tx: Tx(5),
                amount: Amount(10.0),
            })
            .unwrap();
        analysis
            .process_event(&Event::Deposit {
                client: Client(3),
                tx: Tx(6),
                amount: Amount(1.0),
            })
            .unwrap();
        analysis
            .process_event(&Event::Dispute {
                client: Client(3),
                tx: Tx(5),
            })
            .unwrap();
        analysis
            .process_event(&Event::Chargeback {
                client: Client(3),
                tx: Tx(5),
            })
            .unwrap();

        let summary = analysis.summary().collect::<HashSet<AccountSummary>>();

        assert_eq!(
            summary,
            [
                AccountSummary {
                    client: Client(1),
                    available: String::from("1.0000"),
                    held: String::from("1.0000"),
                    total: String::from("2.0000"),
                    locked: false
                },
                AccountSummary {
                    client: Client(2),
                    available: String::from("9.0000"),
                    held: String::from("0.0000"),
                    total: String::from("9.0000"),
                    locked: false
                },
                AccountSummary {
                    client: Client(3),
                    available: String::from("1.0000"),
                    held: String::from("0.0000"),
                    total: String::from("1.0000"),
                    locked: true
                },
            ]
            .into_iter()
            .collect::<HashSet<AccountSummary>>()
        )
    }
}
