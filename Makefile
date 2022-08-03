run:
	cargo run

check:
	cargo check

exec:
	rustc src/string_lib/string_lib.rs

.PHONY: rust, check, exec,