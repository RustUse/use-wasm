#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned when module metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasmModuleError {
    /// The supplied value was empty.
    Empty,
    /// The supplied value contains unsupported characters.
    InvalidName,
    /// The supplied label was not recognized.
    UnknownLabel,
}

impl fmt::Display for WasmModuleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("WebAssembly module value cannot be empty"),
            Self::InvalidName => formatter.write_str("invalid WebAssembly module name"),
            Self::UnknownLabel => formatter.write_str("unknown WebAssembly module label"),
        }
    }
}

impl Error for WasmModuleError {}

fn validate_module_text(value: &str) -> Result<&str, WasmModuleError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(WasmModuleError::Empty);
    }
    if trimmed.chars().any(char::is_control) {
        return Err(WasmModuleError::InvalidName);
    }
    Ok(trimmed)
}

/// Validated module name metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModuleName(String);

impl ModuleName {
    /// Creates a validated module name.
    pub fn new(value: impl AsRef<str>) -> Result<Self, WasmModuleError> {
        validate_module_text(value.as_ref()).map(|value| Self(value.to_owned()))
    }

    /// Returns the stored module name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the wrapper and returns the stored module name.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ModuleName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ModuleName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ModuleName {
    type Err = WasmModuleError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for ModuleName {
    type Error = WasmModuleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Coarse module kind metadata.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModuleKind {
    /// Core Wasm binary module.
    #[default]
    CoreBinary,
    /// Core Wasm text module.
    CoreText,
    /// Component Model component.
    Component,
    /// WIT package metadata.
    WitPackage,
}

impl ModuleKind {
    /// Returns a stable kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreBinary => "core-binary",
            Self::CoreText => "core-text",
            Self::Component => "component",
            Self::WitPackage => "wit-package",
        }
    }
}

impl fmt::Display for ModuleKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ModuleKind {
    type Err = WasmModuleError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(WasmModuleError::Empty);
        }
        match trimmed
            .to_ascii_lowercase()
            .replace(['_', ' '], "-")
            .as_str()
        {
            "core-binary" | "binary" => Ok(Self::CoreBinary),
            "core-text" | "text" | "wat" => Ok(Self::CoreText),
            "component" => Ok(Self::Component),
            "wit-package" | "wit" => Ok(Self::WitPackage),
            _ => Err(WasmModuleError::UnknownLabel),
        }
    }
}

/// Import/export external item kind.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModuleItemKind {
    /// Function item.
    #[default]
    Function,
    /// Table item.
    Table,
    /// Memory item.
    Memory,
    /// Global item.
    Global,
}

/// Module validation status metadata.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ValidationStatus {
    /// Validation has not run.
    #[default]
    NotValidated,
    /// Validation passed.
    Valid,
    /// Validation failed.
    Invalid,
}

impl ValidationStatus {
    /// Returns 'true' when validation passed.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        matches!(self, Self::Valid)
    }
}

/// Module import metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ModuleImport {
    module: String,
    name: String,
    kind: ModuleItemKind,
}

impl ModuleImport {
    /// Creates module import metadata.
    pub fn new(
        module: impl AsRef<str>,
        name: impl AsRef<str>,
        kind: ModuleItemKind,
    ) -> Result<Self, WasmModuleError> {
        Ok(Self {
            module: validate_module_text(module.as_ref())?.to_owned(),
            name: validate_module_text(name.as_ref())?.to_owned(),
            kind,
        })
    }

    /// Returns the import module label.
    #[must_use]
    pub fn module(&self) -> &str {
        &self.module
    }

    /// Returns the import name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the imported item kind.
    #[must_use]
    pub const fn kind(&self) -> ModuleItemKind {
        self.kind
    }
}

/// Module export metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ModuleExport {
    name: String,
    kind: ModuleItemKind,
}

impl ModuleExport {
    /// Creates module export metadata.
    pub fn new(name: impl AsRef<str>, kind: ModuleItemKind) -> Result<Self, WasmModuleError> {
        Ok(Self {
            name: validate_module_text(name.as_ref())?.to_owned(),
            kind,
        })
    }

    /// Returns the export name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the exported item kind.
    #[must_use]
    pub const fn kind(&self) -> ModuleItemKind {
        self.kind
    }
}

/// Module-level metadata collected without parsing or executing a module.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ModuleMetadata {
    name: Option<ModuleName>,
    kind: ModuleKind,
    imports: Vec<ModuleImport>,
    exports: Vec<ModuleExport>,
    validation_status: ValidationStatus,
}

impl ModuleMetadata {
    /// Creates empty module metadata for a kind.
    #[must_use]
    pub const fn new(kind: ModuleKind) -> Self {
        Self {
            name: None,
            kind,
            imports: Vec::new(),
            exports: Vec::new(),
            validation_status: ValidationStatus::NotValidated,
        }
    }

    /// Sets the module name.
    #[must_use]
    pub fn with_name(mut self, name: ModuleName) -> Self {
        self.name = Some(name);
        self
    }

    /// Adds an import.
    #[must_use]
    pub fn with_import(mut self, import: ModuleImport) -> Self {
        self.imports.push(import);
        self
    }

    /// Adds an export.
    #[must_use]
    pub fn with_export(mut self, export: ModuleExport) -> Self {
        self.exports.push(export);
        self
    }

    /// Sets the validation status.
    #[must_use]
    pub const fn with_validation_status(mut self, status: ValidationStatus) -> Self {
        self.validation_status = status;
        self
    }

    /// Returns the module kind.
    #[must_use]
    pub const fn kind(&self) -> ModuleKind {
        self.kind
    }

    /// Returns module imports.
    #[must_use]
    pub fn imports(&self) -> &[ModuleImport] {
        &self.imports
    }

    /// Returns module exports.
    #[must_use]
    pub fn exports(&self) -> &[ModuleExport] {
        &self.exports
    }

    /// Returns validation status metadata.
    #[must_use]
    pub const fn validation_status(&self) -> ValidationStatus {
        self.validation_status
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ModuleExport, ModuleImport, ModuleItemKind, ModuleKind, ModuleMetadata, ModuleName,
        ValidationStatus, WasmModuleError,
    };

    #[test]
    fn validates_module_names_and_kinds() {
        let name = ModuleName::new("example").expect("valid module name");
        let kind = "wat".parse::<ModuleKind>().expect("known module kind");

        assert_eq!(name.as_str(), "example");
        assert_eq!(kind, ModuleKind::CoreText);
        assert_eq!(ModuleName::new("\u{7}"), Err(WasmModuleError::InvalidName));
    }

    #[test]
    fn stores_module_metadata() {
        let metadata = ModuleMetadata::new(ModuleKind::CoreBinary)
            .with_name(ModuleName::new("example").expect("valid module name"))
            .with_import(
                ModuleImport::new("env", "memory", ModuleItemKind::Memory).expect("valid import"),
            )
            .with_export(ModuleExport::new("run", ModuleItemKind::Function).expect("valid export"))
            .with_validation_status(ValidationStatus::Valid);

        assert_eq!(metadata.kind(), ModuleKind::CoreBinary);
        assert_eq!(metadata.imports().len(), 1);
        assert_eq!(metadata.exports().len(), 1);
        assert!(metadata.validation_status().is_valid());
    }
}
