#![warn(clippy::pedantic)]

mod event;
use event::{Event, RawEvent};
mod analysis;
use analysis::Analysis;

use std::{
    env,
    fs::File,
    io::stderr,
    io::{BufReader, Read},
};

fn main() -> eyre::Result<()> {
    let (non_blocking, _guard) = tracing_appender::non_blocking(stderr());
    tracing_subscriber::fmt().with_writer(non_blocking).init();

    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        // TODO: consider replacing open_file call with Builder::path
        .from_reader(open_file()?);

    let mut analysis = Analysis::begin();
    while let Some(raw_event) = reader.deserialize::<RawEvent>().next() {
        let raw_event = raw_event.map_err(eyre::Report::from)?;
        let event = Event::try_from(raw_event)?;

        let _ = analysis
            .process_event(&event)
            .map_err(|err| tracing::error!("analysis error: {}", err));
    }

    Ok(())
}

fn open_file() -> eyre::Result<impl Read> {
    let file_name = env::args()
        .nth(1) // skip executable name and take first effective argument
        .ok_or_else(|| eyre::eyre!("Input file name expected"))?;

    let file = File::open(file_name).map_err(eyre::Report::from)?;

    Ok(BufReader::new(file))
}
