# use-wasm-module

Module-level WebAssembly primitives for RustUse. This crate models names, kinds, imports, exports, metadata, and validation status without parsing or executing modules.

## Example

~~~rust
use use_wasm_module::{ModuleItemKind, ModuleKind, ModuleMetadata, ModuleName};

let metadata = ModuleMetadata::new(ModuleKind::CoreBinary)
    .with_name(ModuleName::new("example").expect("valid module"));

assert_eq!(metadata.kind(), ModuleKind::CoreBinary);
~~~

## Scope

- Module names, kinds, imports, exports, and validation status.
- Small metadata builders and accessors.

## Non-goals

- No module execution.
- No full binary parser.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
