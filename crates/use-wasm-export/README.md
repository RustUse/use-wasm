# use-wasm-export

WebAssembly export primitives for RustUse. This crate models conservative export names, export kinds, and simple exported item metadata.

## Example

~~~rust
use use_wasm_export::{ExportName, ExportedFunction};

let function = ExportedFunction::new(ExportName::new("run").expect("valid name"), 0);

assert_eq!(function.name().as_str(), "run");
assert_eq!(function.index(), 0);
~~~

## Scope

- Export names and kinds.
- Exported function, memory, table, and global metadata.

## Non-goals

- No export resolution.
- No runtime invocation.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
