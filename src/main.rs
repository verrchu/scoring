mod event;
use event::{Event, RawEvent};
mod analysis;
use analysis::Analysis;

use std::{
    env,
    io::{stderr, stdout},
};

use tracing_subscriber::EnvFilter;

fn main() -> eyre::Result<()> {
    let (non_blocking, _guard) = tracing_appender::non_blocking(stderr());
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .init();

    let file_path = env::args()
        .nth(1) // skip executable name and take first effective argument
        .ok_or_else(|| eyre::eyre!("Input file name expected"))?;

    let mut csv_reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(file_path)
        .map_err(|err| eyre::Report::from(err).wrap_err("failed to init csv reader"))?;

    let mut analysis = Analysis::begin();

    while let Some(raw_event) = csv_reader.deserialize::<RawEvent>().next() {
        let raw_event = raw_event.map_err(eyre::Report::from)?;
        let event = Event::try_from(raw_event)?;

        let _ = analysis
            .process_event(&event)
            .map_err(|err| tracing::error!("analysis error: {}", err));
    }

    let mut csv_writer = csv::Writer::from_writer(stdout());

    for account_summary in analysis.summary() {
        csv_writer
            .serialize(account_summary)
            .map_err(|err| eyre::Report::from(err).wrap_err("failed to write csv record"))?;
    }

    csv_writer
        .flush()
        .map_err(|err| eyre::Report::from(err).wrap_err("failed to flush csv writer"))?;

    Ok(())
}
