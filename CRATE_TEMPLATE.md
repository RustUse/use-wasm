# RustUse Wasm crate template

Focused crates in this workspace should stay small, dependency-light, and primitive.

Checklist:

- Use Rust edition 2024 and workspace metadata.
- Include `#![forbid(unsafe_code)]` and README-driven crate docs.
- Prefer explicit error enums.
- Add unit tests for parsing, validation, and display behavior.
- Avoid runtime execution, full parsers, network calls, and external tool shell-outs.
