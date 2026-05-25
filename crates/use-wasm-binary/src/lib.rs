#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::error::Error;

/// WebAssembly binary magic bytes: '\0asm'.
pub const WASM_MAGIC_BYTES: [u8; 4] = [0x00, b'a', b's', b'm'];

/// WebAssembly version 1 encoded as little-endian bytes.
pub const WASM_VERSION_1_BYTES: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

/// The byte length of a minimal WebAssembly binary header.
pub const WASM_BINARY_HEADER_LEN: usize = 8;

/// Coarse WebAssembly binary format marker.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasmBinaryFormat {
    /// Core WebAssembly module binary format.
    #[default]
    CoreModule,
}

impl WasmBinaryFormat {
    /// Returns the stable format label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreModule => "core-module",
        }
    }

    /// Returns the magic bytes associated with this format.
    #[must_use]
    pub const fn magic_bytes(self) -> [u8; 4] {
        match self {
            Self::CoreModule => WASM_MAGIC_BYTES,
        }
    }

    /// Returns the supported version bytes associated with this format.
    #[must_use]
    pub const fn version_bytes(self) -> [u8; 4] {
        match self {
            Self::CoreModule => WASM_VERSION_1_BYTES,
        }
    }
}

impl fmt::Display for WasmBinaryFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Error returned when a WebAssembly binary header is not recognized.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasmBinaryError {
    /// Fewer than eight bytes were provided.
    TooShort,
    /// The first four bytes were not the WebAssembly magic bytes.
    InvalidMagic,
    /// The version bytes do not describe WebAssembly version 1.
    UnsupportedVersion,
}

impl fmt::Display for WasmBinaryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooShort => formatter.write_str("WebAssembly binary header is too short"),
            Self::InvalidMagic => formatter.write_str("missing WebAssembly magic bytes"),
            Self::UnsupportedVersion => {
                formatter.write_str("unsupported WebAssembly binary version")
            },
        }
    }
}

impl Error for WasmBinaryError {}

/// Minimal WebAssembly binary header metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WasmBinaryHeader {
    version: u32,
}

impl WasmBinaryHeader {
    /// Creates header metadata for a known version.
    #[must_use]
    pub const fn new(version: u32) -> Self {
        Self { version }
    }

    /// Returns the decoded WebAssembly binary version.
    #[must_use]
    pub const fn version(self) -> u32 {
        self.version
    }

    /// Returns 'true' for WebAssembly version 1.
    #[must_use]
    pub const fn is_version_1(self) -> bool {
        self.version == 1
    }

    /// Returns the binary format marker for this header.
    #[must_use]
    pub const fn format(self) -> WasmBinaryFormat {
        WasmBinaryFormat::CoreModule
    }
}

/// Returns 'true' when the byte slice starts with the WebAssembly magic bytes.
#[must_use]
pub fn has_wasm_magic(bytes: &[u8]) -> bool {
    bytes.get(..WASM_MAGIC_BYTES.len()) == Some(WASM_MAGIC_BYTES.as_slice())
}

/// Returns 'true' when the byte slice has the magic bytes and version-1 header.
#[must_use]
pub fn looks_like_wasm_binary(bytes: &[u8]) -> bool {
    validate_wasm_header(bytes).is_ok()
}

/// Validates a minimal WebAssembly binary header.
pub fn validate_wasm_header(bytes: &[u8]) -> Result<WasmBinaryHeader, WasmBinaryError> {
    if bytes.len() < WASM_BINARY_HEADER_LEN {
        return Err(WasmBinaryError::TooShort);
    }

    if !has_wasm_magic(bytes) {
        return Err(WasmBinaryError::InvalidMagic);
    }

    let version = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    if version != 1 {
        return Err(WasmBinaryError::UnsupportedVersion);
    }

    Ok(WasmBinaryHeader::new(version))
}

/// Parses a minimal WebAssembly binary header.
pub fn parse_wasm_binary_header(bytes: &[u8]) -> Result<WasmBinaryHeader, WasmBinaryError> {
    validate_wasm_header(bytes)
}

/// Validates a minimal WebAssembly binary header.
pub fn validate_wasm_binary_header(bytes: &[u8]) -> Result<WasmBinaryHeader, WasmBinaryError> {
    validate_wasm_header(bytes)
}

#[cfg(test)]
mod tests {
    use super::{
        WASM_MAGIC_BYTES, WASM_VERSION_1_BYTES, WasmBinaryError, WasmBinaryFormat, has_wasm_magic,
        looks_like_wasm_binary, parse_wasm_binary_header, validate_wasm_binary_header,
        validate_wasm_header,
    };

    #[test]
    fn detects_magic_bytes() {
        assert!(has_wasm_magic(&WASM_MAGIC_BYTES));
        assert!(!has_wasm_magic(b"wat"));
    }

    #[test]
    fn validates_version_one_header() {
        let mut bytes = Vec::from(WASM_MAGIC_BYTES);
        bytes.extend(WASM_VERSION_1_BYTES);
        let header = validate_wasm_header(&bytes).expect("valid version-one header");

        assert_eq!(header.version(), 1);
        assert!(header.is_version_1());
        assert_eq!(header.format(), WasmBinaryFormat::CoreModule);
        assert_eq!(header.format().to_string(), "core-module");
        assert_eq!(WasmBinaryFormat::CoreModule.magic_bytes(), WASM_MAGIC_BYTES);
        assert_eq!(parse_wasm_binary_header(&bytes), Ok(header));
        assert_eq!(validate_wasm_binary_header(&bytes), Ok(header));
        assert!(looks_like_wasm_binary(&bytes));
    }

    #[test]
    fn rejects_invalid_headers() {
        assert_eq!(
            validate_wasm_header(b"\0asm"),
            Err(WasmBinaryError::TooShort)
        );
        assert_eq!(
            validate_wasm_header(b"nope\x01\0\0\0"),
            Err(WasmBinaryError::InvalidMagic)
        );
        assert_eq!(
            validate_wasm_header(b"\0asm\x02\0\0\0"),
            Err(WasmBinaryError::UnsupportedVersion)
        );
    }
}
