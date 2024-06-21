# pulsar

![CI](https://github.com/ethanuppal/pulsar/actions/workflows/ci.yaml/badge.svg)
[![CodeFactor](https://www.codefactor.io/repository/github/ethanuppal/pulsar/badge)](https://www.codefactor.io/repository/github/ethanuppal/pulsar)

Pulsar is a hobby PL, which, for fun, I am implementing both software and hardware backends.
Currently, I am working toward implementing such a hardware backend via the [calyx] infrastructure.

The WIP syntax is a very blatant clone of Swift's with some modifications.
```swift
func increment(x: Int) -> Int {
    return x + 1
}
func main() {
    let input = [1, 2, 3, 4]
    let output = map<1>(increment, input)
}
```

The [calyx] backend is tested e2e via [verilator], a hardware simulation tool.

## Crates

- [`pulsar-lang`](https://crates.io/crates/pulsar-lang): Compiler driver
- [`pulsar-utils`](https://crates.io/crates/pulsar-utils): Utilities for the `pulsar-*` crates
- [`pulsar-frontend`](https://crates.io/crates/pulsar-frontend): Parser/AST and type checking
- [`pulsar-ir`](https://crates.io/crates/pulsar-ir): Structured and unstructured IR
- [`pulsar-backend`](https://crates.io/crates/pulsar-backend): Target emission, e.g., Verilog (via [calyx])

[calyx]: http://calyxir.org
[verilator]: https://www.veripool.org/verilator/
