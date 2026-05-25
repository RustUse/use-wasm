#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned when a WebAssembly value type cannot be parsed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasmValueTypeError {
    /// The supplied label was empty.
    Empty,
    /// The supplied label or byte code is not a known value type.
    Unknown,
}

impl fmt::Display for WasmValueTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("WebAssembly value type cannot be empty"),
            Self::Unknown => formatter.write_str("unknown WebAssembly value type"),
        }
    }
}

impl Error for WasmValueTypeError {}

/// Core WebAssembly value types.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasmValueType {
    /// 32-bit integer.
    #[default]
    I32,
    /// 64-bit integer.
    I64,
    /// 32-bit float.
    F32,
    /// 64-bit float.
    F64,
    /// 128-bit vector.
    V128,
    /// Function reference.
    FuncRef,
    /// External reference.
    ExternRef,
}

impl WasmValueType {
    /// Returns the canonical value type label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::V128 => "v128",
            Self::FuncRef => "funcref",
            Self::ExternRef => "externref",
        }
    }

    /// Returns the binary value type code.
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::I32 => 0x7f,
            Self::I64 => 0x7e,
            Self::F32 => 0x7d,
            Self::F64 => 0x7c,
            Self::V128 => 0x7b,
            Self::FuncRef => 0x70,
            Self::ExternRef => 0x6f,
        }
    }

    /// Returns 'true' for numeric scalar value types.
    #[must_use]
    pub const fn is_numeric(self) -> bool {
        matches!(self, Self::I32 | Self::I64 | Self::F32 | Self::F64)
    }

    /// Returns 'true' for reference value types.
    #[must_use]
    pub const fn is_reference(self) -> bool {
        matches!(self, Self::FuncRef | Self::ExternRef)
    }

    /// Returns 'true' for vector value types.
    #[must_use]
    pub const fn is_vector(self) -> bool {
        matches!(self, Self::V128)
    }
}

impl fmt::Display for WasmValueType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for WasmValueType {
    type Err = WasmValueTypeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(WasmValueTypeError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace(['-', '_'], "");
        match normalized.as_str() {
            "i32" => Ok(Self::I32),
            "i64" => Ok(Self::I64),
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            "v128" => Ok(Self::V128),
            "funcref" => Ok(Self::FuncRef),
            "externref" => Ok(Self::ExternRef),
            _ => Err(WasmValueTypeError::Unknown),
        }
    }
}

impl TryFrom<u8> for WasmValueType {
    type Error = WasmValueTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x7f => Ok(Self::I32),
            0x7e => Ok(Self::I64),
            0x7d => Ok(Self::F32),
            0x7c => Ok(Self::F64),
            0x7b => Ok(Self::V128),
            0x70 => Ok(Self::FuncRef),
            0x6f => Ok(Self::ExternRef),
            _ => Err(WasmValueTypeError::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{WasmValueType, WasmValueTypeError};

    #[test]
    fn parses_value_types() {
        assert_eq!("i32".parse::<WasmValueType>(), Ok(WasmValueType::I32));
        assert_eq!(
            "func-ref".parse::<WasmValueType>(),
            Ok(WasmValueType::FuncRef)
        );
        assert_eq!("".parse::<WasmValueType>(), Err(WasmValueTypeError::Empty));
    }

    #[test]
    fn classifies_and_renders_types() {
        assert!(WasmValueType::I64.is_numeric());
        assert!(WasmValueType::ExternRef.is_reference());
        assert!(WasmValueType::V128.is_vector());
        assert_eq!(WasmValueType::F32.code(), 0x7d);
        assert_eq!(WasmValueType::F64.to_string(), "f64");
    }
}
