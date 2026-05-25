# RustUse/use-wasm

`use-wasm` is a RustUse facade workspace for small, focused WebAssembly primitive crates. It models WebAssembly modules, binary and text metadata, sections, imports, exports, value types, linear memory limits, Rust Wasm targets, WASI, WIT, and Component Model vocabulary without becoming an execution or toolchain layer.

WebAssembly is broader than browser usage. It appears in server runtimes, plugin systems, portable components, edge platforms, and command-oriented system interfaces, so `use-wasm` lives as a top-level RustUse set rather than inside `use-web`.

## Non-goals

`use-wasm` is not:

- a Wasm runtime
- a compiler
- a linker
- an optimizer
- a browser binding layer
- a full Wasm binary parser
- a full WAT, WIT, or Component Model parser
- a replacement for `wasm-bindgen`, `wasm-pack`, `wasmtime`, `wasmer`, or `cargo-component`

The crates avoid runtime execution, network calls, browser-specific bindings, host system APIs, and shelling out to external Wasm tooling.

## Workspace crates

| Crate                | Path                         | Purpose                                                                      |
| -------------------- | ---------------------------- | ---------------------------------------------------------------------------- |
| `use-wasm`           | `crates/use-wasm/`           | Facade over the focused WebAssembly primitive crates                         |
| `use-wasm-module`    | `crates/use-wasm-module/`    | Module names, metadata, imports, exports, and validation status              |
| `use-wasm-binary`    | `crates/use-wasm-binary/`    | Magic bytes, version bytes, binary markers, and header checks                |
| `use-wasm-text`      | `crates/use-wasm-text/`      | WAT identifiers, module names, S-expression markers, and text helpers        |
| `use-wasm-section`   | `crates/use-wasm-section/`   | Known section IDs, labels, parsing, and rendering                            |
| `use-wasm-import`    | `crates/use-wasm-import/`    | Import module/name wrappers, kinds, and imported item metadata               |
| `use-wasm-export`    | `crates/use-wasm-export/`    | Export name wrappers, kinds, and exported item metadata                      |
| `use-wasm-memory`    | `crates/use-wasm-memory/`    | Page counts, memory limits, shared marker, and size conversions              |
| `use-wasm-function`  | `crates/use-wasm-function/`  | Function/type/local indexes, parameter/result lists, and signatures          |
| `use-wasm-value`     | `crates/use-wasm-value/`     | Wasm value type parsing, byte codes, and classification                      |
| `use-wasm-target`    | `crates/use-wasm-target/`    | Common Rust Wasm targets, family, ABI/profile labels, and display helpers    |
| `use-wasm-feature`   | `crates/use-wasm-feature/`   | Common Wasm feature flags and status labels                                  |
| `use-wasm-component` | `crates/use-wasm-component/` | Component, world, interface, import/export, and package-reference primitives |
| `use-wasm-wit`       | `crates/use-wasm-wit/`       | WIT package, namespace, interface, world, type, function, and resource names |
| `use-wasi`           | `crates/use-wasi/`           | Pure WASI version, profile, interface, capability, and permission markers    |

## Relationships

- `use-wasm` models WebAssembly primitives.
- `use-wasi` is a child crate because WASI is a system interface layer for Wasm.
- `use-wasm-component` and `use-wasm-wit` model Component Model and WIT primitives.
- Browser-specific crates can be added later if needed, but they should not dominate v0.1.

## Installation

Use the workspace directly or depend on a Git revision until the first crates.io release is published.

```toml
[dependencies]
use-wasm = { git = "https://github.com/RustUse/use-wasm", rev = "<commit>" }
```

After publication, choose the narrowest focused crate that matches your use case or use the facade when one dependency is more convenient.

```toml
[dependencies]
use-wasm = "0.0.1"
```

## Basic usage

```rust
# #[cfg(feature = "full")]
# {
use use_wasm::{binary, export, import, memory, section, target, value, wasi, wit};

assert!(binary::looks_like_wasm_binary(b"\0asm\x01\0\0\0"));
assert_eq!(section::WasmSectionKind::try_from(5)?.to_string(), "memory");

let limits = memory::MemoryLimits::new(
    memory::WasmPageCount::new(1),
    Some(memory::WasmPageCount::new(2)),
)?;
let value_type: value::WasmValueType = "i64".parse()?;
let import_name = import::ImportName::new("memory")?;
let export_name = export::ExportName::new("run")?;
let wasm_target: target::WasmTarget = "wasm32-wasip1".parse()?;
let world = wit::WitWorldName::new("cli")?;
let capability = wasi::WasiCapabilityLabel::new("filesystem.read")?;

assert_eq!(limits.minimum_pages(), 1);
assert!(value_type.is_numeric());
assert_eq!(import_name.as_str(), "memory");
assert_eq!(export_name.as_str(), "run");
assert_eq!(wasm_target.pointer_width(), 32);
assert_eq!(world.as_str(), "cli");
assert_eq!(capability.as_str(), "filesystem.read");
# }
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Development

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
