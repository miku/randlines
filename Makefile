SHELL := /bin/bash

randlines: src/main.rs
	cargo build --release && cp target/release/randlines .

.PHONY: clippy
clippy:
	cargo clippy

.PHONY: clean
clean:
	rm -f randlines
