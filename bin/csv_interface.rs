use scoring::{Analysis, Event, RawEvent};

use std::{
    env,
    io::{stderr, stdout},
};

use tracing_subscriber::EnvFilter;

fn main() -> eyre::Result<()> {
    // Setup tracing
    //
    // NOTE: Logs are forwarded to stderr.
    // This way they are not mixed out with the program output
    let (non_blocking, _guard) = tracing_appender::non_blocking(stderr());
    // Logs are configured from standard RUST_LOG variable
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .init();

    // Get input file path
    let file_path = env::args()
        .nth(1) // skip executable name and take first effective argument
        .ok_or_else(|| eyre::eyre!("Input file name expected"))?;

    // Init csv reader
    let mut csv_reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(file_path)
        .map_err(|err| eyre::Report::from(err).wrap_err("failed to init csv reader"))?;

    // Init analysis
    let mut analysis = Analysis::begin();

    // Process events sequentially
    while let Some(raw_event) = csv_reader.deserialize::<RawEvent>().next() {
        // Intermediate representation is used for event deserialization
        let raw_event = raw_event.map_err(eyre::Report::from)?;
        let event = Event::try_from(raw_event)?;

        let _ = analysis
            .process_event(&event)
            .map_err(|err| tracing::error!("analysis error: {}", err));
    }

    // Init csv writer
    let mut csv_writer = csv::Writer::from_writer(stdout());

    // Output analysis summary sequentially
    for account_summary in analysis.summary() {
        csv_writer
            .serialize(account_summary)
            .map_err(|err| eyre::Report::from(err).wrap_err("failed to write csv record"))?;
    }

    // Flush csv writer
    csv_writer
        .flush()
        .map_err(|err| eyre::Report::from(err).wrap_err("failed to flush csv writer"))?;

    Ok(())
}
