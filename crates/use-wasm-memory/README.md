# use-wasm-memory

WebAssembly linear memory primitives for RustUse. This crate models page counts, memory limits, shared-memory markers, and page-size conversions.

## Example

~~~rust
use use_wasm_memory::{MemoryLimits, WasmPageCount};

let limits = MemoryLimits::new(WasmPageCount::new(1), Some(WasmPageCount::new(2)))
    .expect("valid limits");

assert_eq!(limits.minimum_pages(), 1);
assert_eq!(limits.maximum_pages(), Some(2));
~~~

## Scope

- Wasm page counts and byte conversions.
- Minimum/maximum memory limits and shared marker.

## Non-goals

- No memory allocation.
- No runtime memory access.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
