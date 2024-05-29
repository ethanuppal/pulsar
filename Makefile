# Copyright (C) 2024 Ethan Uppal. All rights reserved.

BUILD	:= debug

.PHONY: build
build:
	@echo '[INFO] Building project'
	@cargo build
	@echo '$(PWD)/target/$(BUILD)/main "$$@"' > ./main
	@chmod u+x ./main

.PHONY: test
test: build
	@echo '[INFO] Running tests'
	@cargo nextest run --features disable_color

.PHONY: coverage
coverage: build
	@echo '[INFO] Checking coverage'
	@cargo llvm-cov nextest --html
	@open target/llvm-cov/html/index.html

.PHONY: deps
deps:
	@echo '[INFO] Installing dependencies'
	@cargo install cargo-nextest
	@curl -LsSf https://insta.rs/install.sh | sh
	@cargo install cargo-llvm-cov
	@if [ "$(shell uname -s)" = "Linux" ]; then \
		echo '[INFO] Installing Verilator on Ubuntu'; \
		sudo apt-get install -y verilator; \
	elif [ "$(shell uname -s)" = "Darwin" ]; then \
		echo '[INFO] Installing Verilator on macOS'; \
		brew install verilator; \
	fi

.PHONY: ci_check
ci_check:
	@echo "Checking that 'make' works"
	make
	@echo "Checking that './main' works"
	./main
	@echo "Checking that the verilator testing harness works"
	cd tests/calyx-verilog && make N=twice


.PHONY: clean
clean:
	@echo '[INFO] Removing build files'
	@cargo clean

.PHONY: docs
docs:
	@echo '[INFO] Building and viewing documentation'
	@cargo doc --no-deps --open

.PHONY: cloc
cloc:
	@cloc --include-lang=rust --by-file --exclude-dir=target .
