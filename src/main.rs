#![warn(clippy::pedantic)]

mod event;
use event::{Event, RawEvent};
mod score;
use score::Score;

use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

fn main() -> eyre::Result<()> {
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        // TODO: consider replacing open_file call with Builder::path
        .from_reader(open_file()?);

    let mut score = Score::new();
    while let Some(raw_event) = reader.deserialize::<RawEvent>().next() {
        let raw_event = raw_event.map_err(eyre::Report::from)?;
        let event = Event::try_from(raw_event)?;

        score.process_event(&event)?;
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
