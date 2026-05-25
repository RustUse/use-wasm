# use-wasm-value

WebAssembly value type primitives for RustUse. This crate models core scalar, vector, and reference value type labels with parsing and display helpers.

## Example

~~~rust
use use_wasm_value::WasmValueType;

let value_type: WasmValueType = "i64".parse().expect("known value type");

assert!(value_type.is_numeric());
assert_eq!(value_type.to_string(), "i64");
~~~

## Scope

- Core value type labels and byte codes.
- Basic numeric/reference/vector classification.

## Non-goals

- No runtime values.
- No instruction semantics.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
