use rand::{
    distributions::{weighted::WeightedIndex, Distribution},
    rngs::ThreadRng,
    Rng,
};
use structopt::StructOpt;

use scoring::{
    event::{
        wrappers::{Amount, Client, Tx},
        EventType,
    },
    Event, RawEvent,
};

#[derive(Debug, StructOpt)]
struct Args {
    // Max number of accounts to generate events for
    #[structopt(long)]
    accounts: u16,
    // number of events to generate
    #[structopt(long)]
    events: u32,
    // output file
    #[structopt(long)]
    file: String,
}

fn main() {
    let args = Args::from_args();

    assert!(
        (args.accounts as u32) < args.events,
        "accounts number should be lower than events number"
    );

    // Describe desired event type distribution in generated event log
    let (choices, weights) = {
        use EventType::*;

        (
            [Deposit, Withdrawal, Dispute, Resolve, Chargeback],
            [66, 26, 4, 2, 2],
        )
    };

    let wi = WeightedIndex::new(&weights).unwrap();
    let mut rng = rand::thread_rng();

    let mut csv_writer = csv::Writer::from_path(args.file).unwrap();

    // txs start from 2 for some boring reason (just don't think about it)
    for tx in (2..args.events).map(Tx) {
        let event_type = choices[wi.sample(&mut rng)];
        let account = Client(rng.gen_range(1..=args.accounts));
        let event = generate_event(&mut rng, event_type, account, tx);

        csv_writer.serialize(RawEvent::from(event)).unwrap();
    }

    csv_writer.flush().unwrap();
}

// NOTE: This is a very basic generation approach.
// For example when trying to initiate a dispute it chooses account at random
// which gives very little chance of successful dispute
// Withdrawals too have high chance to fail
// This can be fixed but it does the job for now
fn generate_event(rng: &mut ThreadRng, event_type: EventType, client: Client, tx: Tx) -> Event {
    match event_type {
        EventType::Deposit => Event::Deposit {
            client,
            tx,
            amount: Amount(rng.gen_range(0.0..1000.0)),
        },
        EventType::Withdrawal => Event::Withdrawal {
            client,
            tx,
            amount: Amount(rng.gen_range(0.0..1000.0)),
        },
        EventType::Dispute => Event::Dispute {
            client,
            tx: Tx(rng.gen_range(1..tx.0)),
        },
        EventType::Resolve => Event::Resolve {
            client,
            tx: Tx(rng.gen_range(1..tx.0)),
        },
        EventType::Chargeback => Event::Chargeback {
            client,
            tx: Tx(rng.gen_range(1..tx.0)),
        },
    }
}
