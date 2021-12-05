use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    accounts: u64,
    events: u64,
    fiel: String,
}

fn main() {
    let args = Args::from_args();

    assert!(
        args.accounts < args.events,
        "accounts number should be lower than events number"
    );
}
