fmt:
	cargo fmt --all

lint:
	cargo clippy --all -- -D warnings -W clippy::pedantic

test:
	cargo test --all

build:
	cargo build --all

run:
	cargo run -p operator

