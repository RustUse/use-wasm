# Contributing

Thanks for helping improve RustUse/use-wasm. This repository keeps small Rust 2024 crates focused on WebAssembly primitives and avoids runtime, compiler, linker, optimizer, browser binding, network, and host-system behavior.

## Local validation

Run these before opening a pull request:

~~~sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
~~~

For release-readiness work, also run:

~~~sh
cargo test --workspace --no-default-features
cargo check --workspace --all-features --examples
cargo doc --workspace --all-features --no-deps
~~~

## Scope guardrails

- Prefer small validated newtypes, enums, builders, and display helpers.
- Prefer std-only implementations unless a dependency removes real complexity.
- Do not add Wasm execution, full parsing, toolchain shell-outs, network calls, or browser bindings in v0.1.
- Keep errors explicit and typed rather than returning unstructured strings.
