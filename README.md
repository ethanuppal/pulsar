# pulsar

![CI](https://github.com/ethanuppal/pulsar/actions/workflows/ci.yaml/badge.svg)
[![CodeFactor](https://www.codefactor.io/repository/github/ethanuppal/pulsar/badge)](https://www.codefactor.io/repository/github/ethanuppal/pulsar)

Pulsar is a high-level programming language.
Currently, I am working toward implementing a [calyx] backend

## Crates

- [`pulsar-lang`](https://crates.io/crates/pulsar-lang): Compiler driver
- [`pulsar-utils`](https://crates.io/crates/pulsar-utils): Utilities for the `pulsar-*` crates
- [`pulsar-frontend`](https://crates.io/crates/pulsar-frontend): Parser/AST and type checking
- [`pulsar-ir`](https://crates.io/crates/pulsar-ir): Structured and unstructured IR
- [`pulsar-backend`](https://crates.io/crates/pulsar-backend): Target emission, e.g., Verilog (via [calyx])

[calyx]: http://calyxir.org
