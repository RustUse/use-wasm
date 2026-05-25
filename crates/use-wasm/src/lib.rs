#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Thin facade for primitive WebAssembly vocabulary crates.

#[cfg(feature = "wasi")]
pub use use_wasi as wasi;

#[cfg(feature = "binary")]
pub use use_wasm_binary as binary;

#[cfg(feature = "component")]
pub use use_wasm_component as component;

#[cfg(feature = "export")]
pub use use_wasm_export as export;

#[cfg(feature = "feature")]
pub use use_wasm_feature as feature;

#[cfg(feature = "function")]
pub use use_wasm_function as function;

#[cfg(feature = "import")]
pub use use_wasm_import as import;

#[cfg(feature = "memory")]
pub use use_wasm_memory as memory;

#[cfg(feature = "module")]
pub use use_wasm_module as module;

#[cfg(feature = "section")]
pub use use_wasm_section as section;

#[cfg(feature = "target")]
pub use use_wasm_target as target;

#[cfg(feature = "text")]
pub use use_wasm_text as text;

#[cfg(feature = "value")]
pub use use_wasm_value as value;

#[cfg(feature = "wit")]
pub use use_wasm_wit as wit;
