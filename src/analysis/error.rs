use crate::event::wrappers::{Amount, Client, Tx};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("negative amount deposit (client: {0}, tx: {1}, amount: {2})")]
    NegativeAmountDeposit(Client, Tx, Amount),
}
