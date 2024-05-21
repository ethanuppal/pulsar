# Copyright (C) 2024 Ethan Uppal. All rights reserved.

.PHONY: build:
build:
	cargo build

.PHONY: test
test:
	cargo nextest run

.PHONY: deps
deps:
	cargo install cargo-nextest
