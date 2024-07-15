.PHONY: build
build: fmt test lint
	cargo build --release

.PHONY: run
run: fmt test lint
	RUST_BACKTRACE=1 cargo run

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: test
test:
	cargo test

.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings
