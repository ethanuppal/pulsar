# Copyright (C) 2024 Ethan Uppal. All rights reserved.

BUILD	:= debug

.PHONY: build
build:
	@echo '[INFO] Building project'
	@cargo build
	@echo './target/$(BUILD)/main "$$@"' > ./main
	@chmod u+x ./main

.PHONY: test
test: build
	@echo '[INFO] Running tests'
	@cargo nextest run

.PHONY: deps
deps:
	@echo '[INFO] Installing dependencies'
	@cargo install cargo-nextest

.PHONY: clean
clean:
	@echo '[INFO] Removing build files'
	@cargo clean
