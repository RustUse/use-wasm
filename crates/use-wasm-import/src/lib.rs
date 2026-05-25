#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned when import metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasmImportError {
    /// The supplied value was empty.
    Empty,
    /// The supplied name contains unsupported characters.
    InvalidName,
    /// The supplied import kind label was not recognized.
    UnknownKind,
}

impl fmt::Display for WasmImportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("WebAssembly import value cannot be empty"),
            Self::InvalidName => formatter.write_str("invalid WebAssembly import name"),
            Self::UnknownKind => formatter.write_str("unknown WebAssembly import kind"),
        }
    }
}

impl Error for WasmImportError {}

fn validate_import_text(value: &str) -> Result<&str, WasmImportError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(WasmImportError::Empty);
    }
    if trimmed.chars().any(|character| {
        character.is_control()
            || character.is_whitespace()
            || !(character.is_ascii_alphanumeric()
                || matches!(character, '_' | '-' | '.' | '/' | ':' | '$'))
    }) {
        return Err(WasmImportError::InvalidName);
    }
    Ok(trimmed)
}

macro_rules! import_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates validated import text metadata.
            pub fn new(value: impl AsRef<str>) -> Result<Self, WasmImportError> {
                validate_import_text(value.as_ref()).map(|value| Self(value.to_owned()))
            }

            /// Returns the stored text.
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

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = WasmImportError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = WasmImportError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

import_text_newtype!(ImportModuleName);
import_text_newtype!(ImportName);

/// External import kind.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ImportKind {
    /// Function import.
    #[default]
    Function,
    /// Memory import.
    Memory,
    /// Table import.
    Table,
    /// Global import.
    Global,
}

impl ImportKind {
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

impl fmt::Display for ImportKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ImportKind {
    type Err = WasmImportError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(WasmImportError::Empty);
        }
        match trimmed.to_ascii_lowercase().as_str() {
            "function" | "func" => Ok(Self::Function),
            "memory" | "mem" => Ok(Self::Memory),
            "table" => Ok(Self::Table),
            "global" => Ok(Self::Global),
            _ => Err(WasmImportError::UnknownKind),
        }
    }
}

/// Imported function metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ImportedFunction {
    module: ImportModuleName,
    name: ImportName,
    type_index: Option<u32>,
}

impl ImportedFunction {
    /// Creates imported function metadata.
    #[must_use]
    pub const fn new(module: ImportModuleName, name: ImportName) -> Self {
        Self {
            module,
            name,
            type_index: None,
        }
    }

    /// Attaches a function type index.
    #[must_use]
    pub const fn with_type_index(mut self, type_index: u32) -> Self {
        self.type_index = Some(type_index);
        self
    }

    /// Returns the import module name.
    #[must_use]
    pub const fn module(&self) -> &ImportModuleName {
        &self.module
    }

    /// Returns the import field name.
    #[must_use]
    pub const fn name(&self) -> &ImportName {
        &self.name
    }

    /// Returns the optional function type index.
    #[must_use]
    pub const fn type_index(&self) -> Option<u32> {
        self.type_index
    }
}

/// Imported memory metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ImportedMemory {
    module: ImportModuleName,
    name: ImportName,
}

impl ImportedMemory {
    /// Creates imported memory metadata.
    #[must_use]
    pub const fn new(module: ImportModuleName, name: ImportName) -> Self {
        Self { module, name }
    }

    /// Returns the import module name.
    #[must_use]
    pub const fn module(&self) -> &ImportModuleName {
        &self.module
    }

    /// Returns the import field name.
    #[must_use]
    pub const fn name(&self) -> &ImportName {
        &self.name
    }
}

/// Imported table metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ImportedTable {
    module: ImportModuleName,
    name: ImportName,
}

impl ImportedTable {
    /// Creates imported table metadata.
    #[must_use]
    pub const fn new(module: ImportModuleName, name: ImportName) -> Self {
        Self { module, name }
    }
}

/// Imported global metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ImportedGlobal {
    module: ImportModuleName,
    name: ImportName,
    mutable: bool,
}

impl ImportedGlobal {
    /// Creates imported global metadata.
    #[must_use]
    pub const fn new(module: ImportModuleName, name: ImportName, mutable: bool) -> Self {
        Self {
            module,
            name,
            mutable,
        }
    }

    /// Returns 'true' when the imported global is mutable.
    #[must_use]
    pub const fn is_mutable(&self) -> bool {
        self.mutable
    }
}

#[cfg(test)]
mod tests {
    use super::{ImportKind, ImportModuleName, ImportName, ImportedFunction, WasmImportError};

    #[test]
    fn validates_import_names() {
        let module = ImportModuleName::new("wasi:cli").expect("valid module");
        let name = ImportName::new("run").expect("valid import name");

        assert_eq!(module.as_str(), "wasi:cli");
        assert_eq!(name.to_string(), "run");
        assert_eq!(
            ImportName::new("bad name"),
            Err(WasmImportError::InvalidName)
        );
    }

    #[test]
    fn parses_import_kinds_and_metadata() {
        let kind = "func".parse::<ImportKind>().expect("known import kind");
        let function = ImportedFunction::new(
            ImportModuleName::new("env").expect("valid module"),
            ImportName::new("call").expect("valid import"),
        )
        .with_type_index(2);

        assert_eq!(kind, ImportKind::Function);
        assert_eq!(kind.to_string(), "function");
        assert_eq!(function.type_index(), Some(2));
        assert_eq!(function.module().as_str(), "env");
    }
}
