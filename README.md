# scoring - a primitive financial application

## Interface
The program accepts a single command line argument \
which is the name of the input CSV file and outputs the result to stdout. \
The intended way of using the program is like this: \
```cargo run -- input.csv > output.csv```

## Technical notes
* The project is broken down into core library (which is in **src**) and two binaries (both in **bin**).\
The **csv_interface** binary is the one which processes csv input and output \
and uses the core lib internally. The **generate_event_log** binary is used in benchmarks.
* Transactions with negative amount are ignored
* Zero amount transactions are allowed (I can provide cases when it is useful)
* Disputes on withdrawals are ignored
* Amounts are output with fixed 4 decimal places. \
This decision wasn't an easy one to make but made the most sense in the end. \
Basic rounding rules apply to amounts (0.00001 -> 0.0000, 0.99999 -> 1.0000)

## Docs
The project is somewhat covered with **rustdoc**. \
Just run ```make docs``` in the root of the project to browse the docs. \
There are other comments in the code too.

## Unit tests
Just run ```cargo test``` in the root of the project

## Blackbox tests
Blackbox tests are implemented with [bats-core](https://bats-core.readthedocs.io/en/stable/) \
To run them issue ```make bats``` in the root of the project \
**Note that bats depends on bash in yur $PATH**

## Benchmarks
There are some ad-hoc benchmarks. \
They work by generatins some random input CSV file and measuring \
program execution time against this file. \
Just run ```make bench``` in the root of the project.

## Portability
This project has been built, tested and benchmarked against the following official Rust Docker images:
* rust:1.57.0-buster
* rust:1.57.0-alpine
* rust:1.57.0-bullseye

## Codestyle
[clippy](https://github.com/rust-lang/rust-clippy) and [rustfmt](https://github.com/rust-lang/rustfmt) do not complain

## Security
This project has been validated with [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)
