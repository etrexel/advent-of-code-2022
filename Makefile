#SHELL := /bin/bash

all: build test

.PHONY: build
build:
	cargo build --verbose

.PHONY: test
test:
	cargo test --verbose

.PHONY: coverage
coverage:
	cargo install cargo-tarpaulin
	cargo tarpaulin -o Xml

.PHONY: docs
docs:
	cargo doc --no-deps --document-private-items
	echo "<meta http-equiv=\"refresh\" content=\"0; url=aoc\">" > target/doc/index.html

.PHONY: release
release:
	cd /Users/etrexel/Projects && pwd

.PHONY: clean
clean:
	cargo clean
