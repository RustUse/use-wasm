#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned when export metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasmExportError {
    /// The supplied value was empty.
    Empty,
    /// The supplied name contains unsupported characters.
    InvalidName,
    /// The supplied export kind label was not recognized.
    UnknownKind,
}

impl fmt::Display for WasmExportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("WebAssembly export value cannot be empty"),
            Self::InvalidName => formatter.write_str("invalid WebAssembly export name"),
            Self::UnknownKind => formatter.write_str("unknown WebAssembly export kind"),
        }
    }
}

impl Error for WasmExportError {}

fn validate_export_text(value: &str) -> Result<&str, WasmExportError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(WasmExportError::Empty);
    }
    if trimmed.chars().any(|character| {
        character.is_control()
            || character.is_whitespace()
            || !(character.is_ascii_alphanumeric()
                || matches!(character, '_' | '-' | '.' | '/' | ':' | '$'))
    }) {
        return Err(WasmExportError::InvalidName);
    }
    Ok(trimmed)
}

/// Validated WebAssembly export name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExportName(String);

impl ExportName {
    /// Creates a validated export name.
    pub fn new(value: impl AsRef<str>) -> Result<Self, WasmExportError> {
        validate_export_text(value.as_ref()).map(|value| Self(value.to_owned()))
    }

    /// Returns the stored export name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the wrapper and returns the stored string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ExportName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ExportName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ExportName {
    type Err = WasmExportError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for ExportName {
    type Error = WasmExportError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// External export kind.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ExportKind {
    /// Function export.
    #[default]
    Function,
    /// Memory export.
    Memory,
    /// Table export.
    Table,
    /// Global export.
    Global,
}

impl ExportKind {
    /// Returns the stable kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Function => "function",
            Self::Memory => "memory",
            Self::Table => "table",
            Self::Global => "global",
        }
    }
}

impl fmt::Display for ExportKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ExportKind {
    type Err = WasmExportError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(WasmExportError::Empty);
        }
        match trimmed.to_ascii_lowercase().as_str() {
            "function" | "func" => Ok(Self::Function),
            "memory" | "mem" => Ok(Self::Memory),
            "table" => Ok(Self::Table),
            "global" => Ok(Self::Global),
            _ => Err(WasmExportError::UnknownKind),
        }
    }
}

/// Exported function metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExportedFunction {
    name: ExportName,
    index: u32,
}

impl ExportedFunction {
    /// Creates exported function metadata.
    #[must_use]
    pub const fn new(name: ExportName, index: u32) -> Self {
        Self { name, index }
    }

    /// Returns the export name.
    #[must_use]
    pub const fn name(&self) -> &ExportName {
        &self.name
    }

    /// Returns the function index.
    #[must_use]
    pub const fn index(&self) -> u32 {
        self.index
    }
}

/// Exported memory metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExportedMemory {
    name: ExportName,
    index: u32,
}

impl ExportedMemory {
    /// Creates exported memory metadata.
    #[must_use]
    pub const fn new(name: ExportName, index: u32) -> Self {
        Self { name, index }
    }
}

/// Exported table metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExportedTable {
    name: ExportName,
    index: u32,
}

impl ExportedTable {
    /// Creates exported table metadata.
    #[must_use]
    pub const fn new(name: ExportName, index: u32) -> Self {
        Self { name, index }
    }
}

/// Exported global metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExportedGlobal {
    name: ExportName,
    index: u32,
    mutable: bool,
}

impl ExportedGlobal {
    /// Creates exported global metadata.
    #[must_use]
    pub const fn new(name: ExportName, index: u32, mutable: bool) -> Self {
        Self {
            name,
            index,
            mutable,
        }
    }

    /// Returns 'true' when the exported global is mutable.
    #[must_use]
    pub const fn is_mutable(&self) -> bool {
        self.mutable
    }
}

#[cfg(test)]
mod tests {
    use super::{ExportKind, ExportName, ExportedFunction, WasmExportError};

    #[test]
    fn validates_export_names() {
        let name = ExportName::new("run").expect("valid export name");

        assert_eq!(name.as_str(), "run");
        assert_eq!(name.to_string(), "run");
        assert_eq!(
            ExportName::new("bad name"),
            Err(WasmExportError::InvalidName)
        );
    }

    #[test]
    fn parses_export_kinds_and_metadata() {
        let kind = "mem".parse::<ExportKind>().expect("known export kind");
        let function = ExportedFunction::new(ExportName::new("run").expect("valid export"), 3);

        assert_eq!(kind, ExportKind::Memory);
        assert_eq!(kind.to_string(), "memory");
        assert_eq!(function.name().as_str(), "run");
        assert_eq!(function.index(), 3);
    }
}
