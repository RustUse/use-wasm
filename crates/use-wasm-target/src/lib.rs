#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned when a Rust WebAssembly target triple cannot be parsed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasmTargetError {
    /// The supplied target label was empty.
    Empty,
    /// The supplied target label is not in the known target list.
    Unknown,
}

impl fmt::Display for WasmTargetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("WebAssembly target cannot be empty"),
            Self::Unknown => formatter.write_str("unknown WebAssembly target"),
        }
    }
}

impl Error for WasmTargetError {}

/// ABI or profile family for a Rust WebAssembly target.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasmTargetProfile {
    /// Bare Wasm with no host interface implied.
    #[default]
    UnknownUnknown,
    /// WASI Preview 1.
    WasiPreview1,
    /// WASI Preview 1 with threads.
    WasiPreview1Threads,
    /// WASI Preview 2.
    WasiPreview2,
    /// Core Wasm 1.0 profile without std.
    Wasm32V1None,
}

impl WasmTargetProfile {
    /// Returns a stable profile label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnknownUnknown => "unknown-unknown",
            Self::WasiPreview1 => "wasip1",
            Self::WasiPreview1Threads => "wasip1-threads",
            Self::WasiPreview2 => "wasip2",
            Self::Wasm32V1None => "wasm32v1-none",
        }
    }
}

impl fmt::Display for WasmTargetProfile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Common Rust WebAssembly compilation targets.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasmTarget {
    /// 'wasm32-unknown-unknown'.
    #[default]
    Wasm32UnknownUnknown,
    /// 'wasm32-wasip1'.
    Wasm32WasiP1,
    /// 'wasm32-wasip1-threads'.
    Wasm32WasiP1Threads,
    /// 'wasm32-wasip2'.
    Wasm32WasiP2,
    /// 'wasm32v1-none'.
    Wasm32V1None,
}

impl WasmTarget {
    /// Returns the Rust target triple label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Wasm32UnknownUnknown => "wasm32-unknown-unknown",
            Self::Wasm32WasiP1 => "wasm32-wasip1",
            Self::Wasm32WasiP1Threads => "wasm32-wasip1-threads",
            Self::Wasm32WasiP2 => "wasm32-wasip2",
            Self::Wasm32V1None => "wasm32v1-none",
        }
    }

    /// Returns the target family.
    #[must_use]
    pub const fn family(self) -> &'static str {
        "wasm32"
    }

    /// Returns the pointer width in bits.
    #[must_use]
    pub const fn pointer_width(self) -> u8 {
        32
    }

    /// Returns the ABI or profile label.
    #[must_use]
    pub const fn profile(self) -> WasmTargetProfile {
        match self {
            Self::Wasm32UnknownUnknown => WasmTargetProfile::UnknownUnknown,
            Self::Wasm32WasiP1 => WasmTargetProfile::WasiPreview1,
            Self::Wasm32WasiP1Threads => WasmTargetProfile::WasiPreview1Threads,
            Self::Wasm32WasiP2 => WasmTargetProfile::WasiPreview2,
            Self::Wasm32V1None => WasmTargetProfile::Wasm32V1None,
        }
    }

    /// Returns 'true' when the target label explicitly includes thread support.
    #[must_use]
    pub const fn supports_threads(self) -> bool {
        matches!(self, Self::Wasm32WasiP1Threads)
    }
}

impl fmt::Display for WasmTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for WasmTarget {
    type Err = WasmTargetError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(WasmTargetError::Empty);
        }
        match trimmed.to_ascii_lowercase().as_str() {
            "wasm32-unknown-unknown" => Ok(Self::Wasm32UnknownUnknown),
            "wasm32-wasip1" => Ok(Self::Wasm32WasiP1),
            "wasm32-wasip1-threads" => Ok(Self::Wasm32WasiP1Threads),
            "wasm32-wasip2" => Ok(Self::Wasm32WasiP2),
            "wasm32v1-none" => Ok(Self::Wasm32V1None),
            _ => Err(WasmTargetError::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{WasmTarget, WasmTargetError, WasmTargetProfile};

    #[test]
    fn parses_common_targets() {
        assert_eq!(
            "wasm32-wasip1".parse::<WasmTarget>(),
            Ok(WasmTarget::Wasm32WasiP1)
        );
        assert_eq!("".parse::<WasmTarget>(), Err(WasmTargetError::Empty));
    }

    #[test]
    fn exposes_target_metadata() {
        let target = WasmTarget::Wasm32WasiP1Threads;

        assert_eq!(target.family(), "wasm32");
        assert_eq!(target.pointer_width(), 32);
        assert_eq!(target.profile(), WasmTargetProfile::WasiPreview1Threads);
        assert!(target.supports_threads());
        assert_eq!(target.to_string(), "wasm32-wasip1-threads");
    }
}
