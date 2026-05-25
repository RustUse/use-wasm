# Releasing

This workspace follows the RustUse first-publish pattern: publish focused crates first, wait for registry propagation, then publish the `use-wasm` facade.

## First publish order

1. `use-wasm-module`
1. `use-wasm-binary`
1. `use-wasm-text`
1. `use-wasm-section`
1. `use-wasm-import`
1. `use-wasm-export`
1. `use-wasm-memory`
1. `use-wasm-value`
1. `use-wasm-function`
1. `use-wasm-target`
1. `use-wasm-feature`
1. `use-wasm-component`
1. `use-wasm-wit`
1. `use-wasi`
1. `use-wasm`

Use dry runs until every package is ready:

```sh
cargo publish --dry-run --allow-dirty -p use-wasm-binary
cargo publish --dry-run --allow-dirty -p use-wasm
```

Do not publish from this scaffold unless explicitly requested.
