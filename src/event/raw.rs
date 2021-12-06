use serde::{Deserialize, Serialize};

use super::{
    wrappers::{Amount, Client, Tx},
    Event, EventType,
};

/// Utility struct which primary purpose is to be an
/// intermediate representation during (de)serialization.
///
/// One of the cases is limitations of the [csv] library to deserialize
/// internally tagged enums ([issue](https://github.com/BurntSushi/rust-csv/issues/211))
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RawEvent {
    #[serde(rename = "type")]
    ty: EventType,
    client: Client,
    tx: Tx,
    amount: Option<Amount>,
}

impl TryFrom<RawEvent> for Event {
    type Error = eyre::Report;

    fn try_from(raw: RawEvent) -> eyre::Result<Event> {
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
                    amount,
                })
            }
            EventType::Withdrawal => {
                let amount = raw.amount.ok_or_else(|| {
                    eyre::eyre!("Withdrawal has no 'amount' specified: (tx: {})", raw.tx.0)
                })?;

                Ok(Self::Withdrawal {
                    client: raw.client,
                    tx: raw.tx,
                    amount,
                })
            }
        }
    }
}

impl From<Event> for RawEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::Chargeback { client, tx } => Self {
                ty: EventType::Chargeback,
                client,
                tx,
                amount: None,
            },
            Event::Dispute { client, tx } => Self {
                ty: EventType::Dispute,
                client,
                tx,
                amount: None,
            },
            Event::Resolve { client, tx } => Self {
                ty: EventType::Resolve,
                client,
                tx,
                amount: None,
            },
            Event::Deposit { client, tx, amount } => Self {
                ty: EventType::Deposit,
                client,
                tx,
                amount: Some(amount),
            },
            Event::Withdrawal { client, tx, amount } => Self {
                ty: EventType::Withdrawal,
                client,
                tx,
                amount: Some(amount),
            },
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
