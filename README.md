# pulsar

![CI](https://github.com/ethanuppal/pulsar/actions/workflows/ci.yaml/badge.svg)
[![CodeFactor](https://www.codefactor.io/repository/github/ethanuppal/pulsar/badge)](https://www.codefactor.io/repository/github/ethanuppal/pulsar)

Pulsar is a high-level programming language for building hardware accelerators.
Currently, I am working toward implementing a [calyx] backend.
It is currently a very vanilla language because I have not yet determined how it should specialize (but all ADLs in practice are DSLs).

The syntax is a very blatant clone of Swift's with some modifications, and it's also very much a WIP.
```swift
func increment(x: Int) -> Int {
    return x + 1
}
func main() {
    let input = [1, 2, 3, 4]
    let output = map<1>(increment, input)
}
```

The compiler is tested e2e via [verilator], a hardware simulation tool.

## Crates

- [`pulsar-lang`](https://crates.io/crates/pulsar-lang): Compiler driver
- [`pulsar-utils`](https://crates.io/crates/pulsar-utils): Utilities for the `pulsar-*` crates
- [`pulsar-frontend`](https://crates.io/crates/pulsar-frontend): Parser/AST and type checking
- [`pulsar-ir`](https://crates.io/crates/pulsar-ir): Structured and unstructured IR
- [`pulsar-backend`](https://crates.io/crates/pulsar-backend): Target emission, e.g., Verilog (via [calyx])

[calyx]: http://calyxir.org
[verilator]: https://www.veripool.org/verilator/
