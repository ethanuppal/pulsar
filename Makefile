# Copyright (C) 2024 Ethan Uppal. All rights reserved.

BUILD	:= debug

ifeq ($(shell uname), Darwin)
    OPEN := $(shell which open)
else
    OPEN := $(shell which xdg-open)
endif

.PHONY: build
build:
	@echo '[INFO] Building project'
	@cargo build
	@echo 'cargo run -- $$@' > ./main
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
	@echo '[INFO] Checking verilator version'
	verilator --version

.PHONY: ci_install_calyx
ci_install_calyx:
	@echo '[INFO] Installing calyx'
	if [ ! -d "$(HOME)/calyx" ]; then \
		cd $(HOME) && git clone https://github.com/calyxir/calyx.git; \
	fi
	cd $(HOME)/calyx && cargo build
	cd $(HOME)/calyx && ./target/debug/calyx --version
	cd $(HOME)/calyx && python -m pip install flit
	cd $(HOME)/calyx && cd calyx-py && flit install -s && cd -
	cd $(HOME)/calyx && flit -f fud/pyproject.toml install -s --deps production
	fud config --create global.root $(HOME)/calyx

.PHONY: ci_check
ci_check:
	@echo "Checking that 'make' works"
	make
	@echo "Checking that the verilator testing harness works"
	cd tests/calyx-verilog && make N=twice
	cd tests/calyx-verilog && make N=map
# @echo "Checking that './main' works"
# ./main 1>/dev/null 2>/dev/null || exit 0

.PHONY: clean
clean:
	@echo '[INFO] Removing build files'
	@cargo clean

.PHONY: docs
docs:
	@echo '[INFO] Building and viewing documentation'
	@cargo doc -p pulsar-frontend -p pulsar-ir -p pulsar-backend -p pulsar-utils -p pulsar --no-deps
		

.PHONY: cloc
cloc:
	@cloc --include-lang=rust --by-file --exclude-dir=target .
