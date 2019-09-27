# Prerequisites

## Install Rust using rustup

https://www.rust-lang.org/tools/install

## Install Diesel CLI for PostgreSQL

`$ cargo install diesel_cli --no-default-features --features postgres`

Was missing -lpq when compiling diesel_cli. Fixed by installing `libpq-dev`.
