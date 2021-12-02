mod event;

use event::Event;

use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

fn main() -> eyre::Result<()> {
    let file = open_file()?;
    let mut reader = csv::ReaderBuilder::new().from_reader(BufReader::new(file));

    while let Some(record) = reader.deserialize::<Event>().next() {
        let event = record.map_err(eyre::Report::from)?;
    }

    Ok(())
}

fn open_file() -> eyre::Result<impl Read> {
    let file_name = env::args()
        .next()
        .ok_or(eyre::eyre!("Input file name expected"))?;

    File::open(file_name).map_err(eyre::Report::from)
}
