# use-wasm-wit

WIT primitive utilities for RustUse. This crate models package, namespace, interface, world, type, function, and resource names with conservative validation.

## Example

~~~rust
use use_wasm_wit::{WitPackageName, WitWorldName};

let package = WitPackageName::new("wasi:cli@0.2.0").expect("valid package");
let world = WitWorldName::new("command").expect("valid world");

assert_eq!(package.as_str(), "wasi:cli@0.2.0");
assert_eq!(world.as_str(), "command");
~~~

## Scope

- WIT package and identifier wrappers.
- Conservative validation and display helpers.

## Non-goals

- No full WIT parser.
- No package resolution.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
