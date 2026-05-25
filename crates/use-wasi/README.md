# use-wasi

WASI primitive utilities for RustUse. This crate models versions, profiles, interface names, capability labels, and permission markers without calling host system APIs.

## Example

~~~rust
use use_wasi::{FilesystemPermission, WasiCapabilityLabel, WasiVersion};

let version: WasiVersion = "wasip2".parse().expect("known version");
let capability = WasiCapabilityLabel::new("filesystem.read").expect("valid capability");

assert_eq!(version.to_string(), "preview2");
assert_eq!(capability.as_str(), FilesystemPermission::Read.as_str());
~~~

## Scope

- WASI version and profile labels.
- Capability and permission marker primitives.

## Non-goals

- No host system API calls.
- No WASI runtime behavior.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
