# use-wasm-function

WebAssembly function metadata primitives for RustUse. This crate models indexes, value type labels, parameter lists, result lists, and signatures without instruction execution.

## Example

~~~rust
use use_wasm_function::{FunctionSignature, FunctionValueType, ParameterList, ResultList};

let signature = FunctionSignature::new(
    ParameterList::new(vec![FunctionValueType::I32]),
    ResultList::new(vec![FunctionValueType::I64]),
);

assert_eq!(signature.params().len(), 1);
~~~

## Scope

- Function, type, and local indexes.
- Parameter/result lists and signature metadata.

## Non-goals

- No instruction execution.
- No function body parsing.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
