# use-wasm-target

Rust/WebAssembly target primitives for RustUse. This crate models common Rust Wasm target labels with family, pointer width, and profile helpers.

## Example

~~~rust
use use_wasm_target::WasmTarget;

let target: WasmTarget = "wasm32-wasip1".parse().expect("known target");

assert_eq!(target.family(), "wasm32");
assert_eq!(target.pointer_width(), 32);
~~~

## Scope

- Common Rust Wasm targets.
- Target family, pointer width, and profile labels.

## Non-goals

- No cargo invocation.
- No target installation checks.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
