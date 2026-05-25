# use-wasm-feature

WebAssembly feature primitives for RustUse. This crate models common feature flags and coarse status labels.

## Example

~~~rust
use use_wasm_feature::{WasmFeature, WasmFeatureStatus};

let feature: WasmFeature = "bulk memory".parse().expect("known feature");

assert_eq!(feature.status(), WasmFeatureStatus::Stable);
assert_eq!(feature.to_string(), "bulk-memory");
~~~

## Scope

- Common Wasm feature flags.
- Stable and experimental status labels.

## Non-goals

- No feature detection against runtimes.
- No compiler flag management.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
