/// provides [Analysis] which is the heart of this crate
pub mod analysis;
/// provides [Event] which is what [Analysis] operates on
pub mod event;

pub use analysis::{AccountSummary, Analysis, AnalysisError, AnalysisResult, AnalysisSummary};
pub use event::{Event, RawEvent};
