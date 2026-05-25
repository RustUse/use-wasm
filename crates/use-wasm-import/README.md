# use-wasm-import

WebAssembly import primitives for RustUse. This crate models conservative import names, import kinds, and simple imported item metadata.

## Example

~~~rust
use use_wasm_import::{ImportModuleName, ImportName, ImportedFunction};

let function = ImportedFunction::new(
    ImportModuleName::new("env").expect("valid module"),
    ImportName::new("call").expect("valid name"),
);

assert_eq!(function.module().as_str(), "env");
~~~

## Scope

- Import module and field names.
- Import kinds and imported item metadata.

## Non-goals

- No type-section resolution.
- No module linking.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
