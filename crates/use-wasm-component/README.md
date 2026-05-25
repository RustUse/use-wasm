# use-wasm-component

WebAssembly Component Model primitives for RustUse. This crate models component, world, interface, package reference, import, and export metadata without encoding or decoding components.

## Example

~~~rust
use use_wasm_component::{ComponentImport, ComponentItemKind, InterfaceName};

let import = ComponentImport::new(
    InterfaceName::new("filesystem").expect("valid interface"),
    ComponentItemKind::Interface,
);

assert_eq!(import.kind(), ComponentItemKind::Interface);
~~~

## Scope

- Component, world, interface, and package-reference names.
- Component import and export metadata.

## Non-goals

- No component encoding.
- No component decoding.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
