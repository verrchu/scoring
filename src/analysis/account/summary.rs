use serde::{Deserialize, Serialize};

use crate::event::wrappers::Client;

/// Represent resulting account state \
/// **Note that amounts are represented as strings.
/// This is due to the requirement of max precision
/// of 4 decimal places in resulting amounts which is
/// easily achievable by opting out of floats in favor of strings**
/// ([amount][crate::event::wrappers::Amount] renders fixed 4 decimal places in its
/// [std::fmt::Display] implementation)
///
/// # Example
/// ```
/// use scoring::{
///     event::wrappers::{Amount, Client, Tx},
///     AccountSummary, Analysis, Event,
/// };
///
/// let mut analysis = Analysis::begin();
///
/// let event = Event::Deposit { client: Client(1), tx: Tx(1), amount: Amount(1.0) };
/// analysis.process_event(&event);
///
/// let summary = analysis.summary();
///
/// assert_eq!(
///     summary.collect::<Vec<AccountSummary>>(),
///     vec![AccountSummary {
///         client: Client(1),
///         locked: false,
///         available: "1.0000".to_string(),
///         held: "0.0000".to_string(),
///         total: "1.0000".to_string(),
///     }]
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Summary {
    /// Client ID
    pub client: Client,
    /// Available amount
    pub available: String,
    /// Held amount (due to disputes in progress)
    pub held: String,
    /// Total amount (available + held)
    pub total: String,
    /// Whether account is locked (due to a chargeback)
    pub locked: bool,
}
