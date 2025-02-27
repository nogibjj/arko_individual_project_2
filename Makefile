install:
	cargo install --path .

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run --release

build:
	cargo build --release

release:
	cargo build --release

all: format lint test run
