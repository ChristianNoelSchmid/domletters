
# Taken from https://stackoverflow.com/a/43566158
SHELL := /bin/bash

build: ./program ./program/Cargo.toml ./program/src/lib.rs ./program/examples/domletters.rs;
	# Taken from https://github.com/rust-lang/rustup/issues/297#issuecomment-444818896
	curl https://sh.rustup.rs -sSf | sh -s -- -y; \
	source $(HOME)/.cargo/env; \
	cd program; \
	cargo build --example domletters --release; \
	mv target/release/examples/domletters ..

clean: ./program;
	rm -f domletters
	rm -rf program/target