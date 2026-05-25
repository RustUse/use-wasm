# use-wasm-text

WAT/WebAssembly text-format primitives for RustUse. This crate provides small identifier and text-shape helpers without implementing a full WAT parser.

## Example

~~~rust
use use_wasm_text::{WatIdentifier, looks_like_wat_module};

let identifier = WatIdentifier::new("$run").expect("valid WAT identifier");

assert_eq!(identifier.as_str(), "$run");
assert!(looks_like_wat_module("(module (func))"));
~~~

## Scope

- WAT identifiers, text module names, and S-expression markers.
- Basic text-format shape checks.

## Non-goals

- No full WAT parser.
- No instruction decoding.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
