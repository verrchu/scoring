# scoring - a primitive financial application

## Interface
The program accepts a single command line argument \
which is the name of the input CSV file and outputs the result to stdout. \
The intended way of using the program is like this: \
```cargo run -- input.csv > output.csv```

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
