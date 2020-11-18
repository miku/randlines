SHELL := /bin/bash

randlines: src/main.rs
	cargo build --release && cp target/release/randlines .

.PHONY: clean
clean:
	rm -f randlines
