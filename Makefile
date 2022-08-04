run:
	cargo run

check:
	cargo check

test:
	cargo test

rustc_doc:
	rustup doc

cargo_doc:
	cargo doc

fmt:
	cargo fmt

clippy:
	cargo clippy

.PHONY: rust, check, test, rust_doc, cargo_doc, fmt, clippy