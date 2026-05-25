# use-wasm-binary

WebAssembly binary format primitives for RustUse. This crate checks magic bytes and version bytes without parsing full Wasm binaries.

## Example

~~~rust
use use_wasm_binary::{looks_like_wasm_binary, validate_wasm_header};

let bytes = b"\0asm\x01\0\0\0";
let header = validate_wasm_header(bytes).expect("valid header");

assert!(looks_like_wasm_binary(bytes));
assert_eq!(header.version(), 1);
~~~

## Scope

- Magic bytes, version bytes, and minimal header validation.
- Helpers for byte slices that look like Wasm binaries.

## Non-goals

- No full binary decoding.
- No validation beyond the header.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
