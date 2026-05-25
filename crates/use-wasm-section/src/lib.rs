#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned when a WebAssembly section label or ID is invalid.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasmSectionError {
    /// The supplied section label was empty.
    Empty,
    /// The supplied section ID is not assigned by the core WebAssembly format.
    UnknownId,
    /// The supplied section label was not recognized.
    UnknownName,
}

impl fmt::Display for WasmSectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("WebAssembly section label cannot be empty"),
            Self::UnknownId => formatter.write_str("unknown WebAssembly section ID"),
            Self::UnknownName => formatter.write_str("unknown WebAssembly section label"),
        }
    }
}

impl Error for WasmSectionError {}

/// Known WebAssembly section kinds.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasmSectionKind {
    /// Section ID 0.
    #[default]
    Custom,
    /// Section ID 1.
    Type,
    /// Section ID 2.
    Import,
    /// Section ID 3.
    Function,
    /// Section ID 4.
    Table,
    /// Section ID 5.
    Memory,
    /// Section ID 6.
    Global,
    /// Section ID 7.
    Export,
    /// Section ID 8.
    Start,
    /// Section ID 9.
    Element,
    /// Section ID 10.
    Code,
    /// Section ID 11.
    Data,
    /// Section ID 12.
    DataCount,
}

impl WasmSectionKind {
    /// Returns the numeric section ID.
    #[must_use]
    pub const fn id(self) -> u8 {
        match self {
            Self::Custom => 0,
            Self::Type => 1,
            Self::Import => 2,
            Self::Function => 3,
            Self::Table => 4,
            Self::Memory => 5,
            Self::Global => 6,
            Self::Export => 7,
            Self::Start => 8,
            Self::Element => 9,
            Self::Code => 10,
            Self::Data => 11,
            Self::DataCount => 12,
        }
    }

    /// Returns the stable section label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Custom => "custom",
            Self::Type => "type",
            Self::Import => "import",
            Self::Function => "function",
            Self::Table => "table",
            Self::Memory => "memory",
            Self::Global => "global",
            Self::Export => "export",
            Self::Start => "start",
            Self::Element => "element",
            Self::Code => "code",
            Self::Data => "data",
            Self::DataCount => "data-count",
        }
    }

    /// Returns 'true' for custom sections.
    #[must_use]
    pub const fn is_custom(self) -> bool {
        matches!(self, Self::Custom)
    }
}

impl fmt::Display for WasmSectionKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl TryFrom<u8> for WasmSectionKind {
    type Error = WasmSectionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Custom),
            1 => Ok(Self::Type),
            2 => Ok(Self::Import),
            3 => Ok(Self::Function),
            4 => Ok(Self::Table),
            5 => Ok(Self::Memory),
            6 => Ok(Self::Global),
            7 => Ok(Self::Export),
            8 => Ok(Self::Start),
            9 => Ok(Self::Element),
            10 => Ok(Self::Code),
            11 => Ok(Self::Data),
            12 => Ok(Self::DataCount),
            _ => Err(WasmSectionError::UnknownId),
        }
    }
}

impl FromStr for WasmSectionKind {
    type Err = WasmSectionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(WasmSectionError::Empty);
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
            "custom" => Ok(Self::Custom),
            "type" => Ok(Self::Type),
            "import" => Ok(Self::Import),
            "function" => Ok(Self::Function),
            "table" => Ok(Self::Table),
            "memory" => Ok(Self::Memory),
            "global" => Ok(Self::Global),
            "export" => Ok(Self::Export),
            "start" => Ok(Self::Start),
            "element" => Ok(Self::Element),
            "code" => Ok(Self::Code),
            "data" => Ok(Self::Data),
            "data-count" | "datacount" => Ok(Self::DataCount),
            _ => Err(WasmSectionError::UnknownName),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{WasmSectionError, WasmSectionKind};

    #[test]
    fn parses_section_ids() {
        assert_eq!(WasmSectionKind::try_from(0), Ok(WasmSectionKind::Custom));
        assert_eq!(
            WasmSectionKind::try_from(12),
            Ok(WasmSectionKind::DataCount)
        );
        assert_eq!(
            WasmSectionKind::try_from(99),
            Err(WasmSectionError::UnknownId)
        );
    }

    #[test]
    fn parses_and_renders_labels() {
        let section = "data count"
            .parse::<WasmSectionKind>()
            .expect("known section");

        assert_eq!(section, WasmSectionKind::DataCount);
        assert_eq!(section.id(), 12);
        assert_eq!(section.to_string(), "data-count");
        assert!(!section.is_custom());
    }
}
