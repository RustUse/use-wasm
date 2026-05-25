# use-wasm-section

WebAssembly section primitives for RustUse. This crate models known section IDs and labels with parsing and display helpers.

## Example

~~~rust
use use_wasm_section::WasmSectionKind;

let section = WasmSectionKind::try_from(5).expect("known section");

assert_eq!(section.to_string(), "memory");
assert_eq!(section.id(), 5);
~~~

## Scope

- Known core section IDs and labels.
- Parsing from IDs and display labels.

## Non-goals

- No section payload parsing.
- No binary reader.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
