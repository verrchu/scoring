pub mod wrappers;

use wrappers::{Amount, Client, Tx};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Event {
    Chargeback {
        client: Client,
        tx: Tx,
    },
    Deposit {
        client: Client,
        tx: Tx,
        amount: Amount,
    },
    Dispute {
        client: Client,
        tx: Tx,
    },
    Resolve {
        client: Client,
        tx: Tx,
    },
    Withdrawal {
        client: Client,
        tx: Tx,
        amount: Amount,
    },
}

// Unfortunately csv crate can not deserialize rows directly to internally
// tagged enums (https://github.com/BurntSushi/rust-csv/issues/211).
// Therefore intermadiate struct is used for bridging purpose.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RawEvent {
    #[serde(rename = "type")]
    ty: EventType,
    client: Client,
    tx: Tx,
    amount: Option<Amount>,
}

// This enum is used only for transition from RawEvent to Event
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Chargeback,
    Deposit,
    Dispute,
    Resolve,
    Withdrawal,
}

impl Event {
    pub fn from_raw(raw: RawEvent) -> eyre::Result<Self> {
        match raw.ty {
            EventType::Chargeback => Ok(Self::Chargeback {
                client: raw.client,
                tx: raw.tx,
            }),
            EventType::Dispute => Ok(Self::Dispute {
                client: raw.client,
                tx: raw.tx,
            }),
            EventType::Resolve => Ok(Self::Resolve {
                client: raw.client,
                tx: raw.tx,
            }),
            EventType::Deposit => {
                let amount = raw.amount.ok_or_else(|| {
                    eyre::eyre!("Deposit has no 'amount' specified: (tx: {})", raw.tx.0)
                })?;

                Ok(Self::Deposit {
                    client: raw.client,
                    tx: raw.tx,
                    amount: amount.round(),
                })
            }
            EventType::Withdrawal => {
                let amount = raw.amount.ok_or_else(|| {
                    eyre::eyre!("Withdrawal has no 'amount' specified: (tx: {})", raw.tx.0)
                })?;

                Ok(Self::Withdrawal {
                    client: raw.client,
                    tx: raw.tx,
                    amount: amount.round(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_matches::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_csv_deserialize() {
        let data = r#"
type ,client ,tx ,amount
deposit, 1,1, 2.0 
withdrawal,1,2,1.0
        "#;

        let mut reader = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_reader(data.trim().as_bytes());
        let events = reader
            .deserialize::<RawEvent>()
            .collect::<Result<Vec<_>, _>>();

        assert_matches!(events, Ok(_));

        let events = events.unwrap();

        assert_eq!(events.len(), 2);
        assert_eq!(
            events,
            vec![
                RawEvent {
                    ty: EventType::Deposit,
                    client: Client(1),
                    tx: Tx(1),
                    amount: Some(Amount(2.0))
                },
                RawEvent {
                    ty: EventType::Withdrawal,
                    client: Client(1),
                    tx: Tx(2),
                    amount: Some(Amount(1.0))
                }
            ]
        );
    }
}
