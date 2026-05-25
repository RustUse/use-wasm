#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned when a WebAssembly feature label cannot be parsed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasmFeatureError {
    /// The supplied feature label was empty.
    Empty,
    /// The supplied feature label is not in the known feature list.
    Unknown,
}

impl fmt::Display for WasmFeatureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("WebAssembly feature label cannot be empty"),
            Self::Unknown => formatter.write_str("unknown WebAssembly feature label"),
        }
    }
}

impl Error for WasmFeatureError {}

/// Coarse stability status for feature labels.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasmFeatureStatus {
    /// Broadly implemented or standardized enough for stable metadata labels.
    #[default]
    Stable,
    /// Useful, but still treated as experimental by this primitive set.
    Experimental,
    /// Historical or discouraged label.
    Deprecated,
}

impl WasmFeatureStatus {
    /// Returns a stable status label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Experimental => "experimental",
            Self::Deprecated => "deprecated",
        }
    }
}

impl fmt::Display for WasmFeatureStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Common WebAssembly feature flags.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasmFeature {
    /// SIMD instructions.
    #[default]
    Simd,
    /// Threading primitives.
    Threads,
    /// Reference types.
    ReferenceTypes,
    /// Bulk memory operations.
    BulkMemory,
    /// Tail calls.
    TailCalls,
    /// Exception handling.
    Exceptions,
    /// Garbage collection proposal family.
    Gc,
    /// 64-bit memory indexes.
    Memory64,
    /// Multiple return values.
    MultiValue,
    /// Component Model support.
    ComponentModel,
}

impl WasmFeature {
    /// Returns the canonical feature label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Simd => "simd",
            Self::Threads => "threads",
            Self::ReferenceTypes => "reference-types",
            Self::BulkMemory => "bulk-memory",
            Self::TailCalls => "tail-calls",
            Self::Exceptions => "exceptions",
            Self::Gc => "gc",
            Self::Memory64 => "memory64",
            Self::MultiValue => "multi-value",
            Self::ComponentModel => "component-model",
        }
    }

    /// Returns the coarse feature status.
    #[must_use]
    pub const fn status(self) -> WasmFeatureStatus {
        match self {
            Self::Exceptions | Self::Gc | Self::Memory64 | Self::TailCalls => {
                WasmFeatureStatus::Experimental
            },
            Self::Simd
            | Self::Threads
            | Self::ReferenceTypes
            | Self::BulkMemory
            | Self::MultiValue
            | Self::ComponentModel => WasmFeatureStatus::Stable,
        }
    }

    /// Returns 'true' when the status is stable.
    #[must_use]
    pub const fn is_stable(self) -> bool {
        matches!(self.status(), WasmFeatureStatus::Stable)
    }
}

impl fmt::Display for WasmFeature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for WasmFeature {
    type Err = WasmFeatureError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(WasmFeatureError::Empty);
        }
        let normalized: String = trimmed
            .chars()
            .map(|character| {
                if character == '_' || character.is_whitespace() {
                    '-'
                } else {
                    character.to_ascii_lowercase()
                }
            })
            .collect();
        match normalized.as_str() {
            "simd" => Ok(Self::Simd),
            "threads" => Ok(Self::Threads),
            "reference-types" => Ok(Self::ReferenceTypes),
            "bulk-memory" => Ok(Self::BulkMemory),
            "tail-calls" => Ok(Self::TailCalls),
            "exceptions" => Ok(Self::Exceptions),
            "gc" => Ok(Self::Gc),
            "memory64" => Ok(Self::Memory64),
            "multi-value" => Ok(Self::MultiValue),
            "component-model" => Ok(Self::ComponentModel),
            _ => Err(WasmFeatureError::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{WasmFeature, WasmFeatureError, WasmFeatureStatus};

    #[test]
    fn parses_feature_labels() {
        assert_eq!(
            "bulk memory".parse::<WasmFeature>(),
            Ok(WasmFeature::BulkMemory)
        );
        assert_eq!("".parse::<WasmFeature>(), Err(WasmFeatureError::Empty));
    }

    #[test]
    fn exposes_status_labels() {
        assert!(WasmFeature::Simd.is_stable());
        assert_eq!(
            WasmFeature::Memory64.status(),
            WasmFeatureStatus::Experimental
        );
        assert_eq!(WasmFeature::ComponentModel.to_string(), "component-model");
        assert_eq!(WasmFeatureStatus::Stable.to_string(), "stable");
    }
}
