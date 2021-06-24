SHELL := /bin/bash

build: ./program ./program/Cargo.toml ./program/src/lib.rs ./program/examples/domletters.rs;
	curl https://sh.rustup.rs -sSf | sh -s -- -y; \
	source $(HOME)/.cargo/env; \
	cd program; \
	cargo build --example domletters --release; \
	mv target/release/examples/domletters ..

clean: ./program;
	rm -f domletters
	rm -rf program/target