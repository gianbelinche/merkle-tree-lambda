build:
	cargo build

example:
	cargo run --example main

test:
	cargo test

lint:
	cargo fmt
	cargo clippy -- -D clippy::nursery -D clippy::all -D clippy::complexity -A clippy::use_self
