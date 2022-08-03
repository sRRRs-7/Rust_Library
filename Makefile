run:
	cargo run

check:
	cargo check

test:
	cargo test

exec:
	rustc src/string_lib/string_lib.rs

.PHONY: rust, check, test, exec,