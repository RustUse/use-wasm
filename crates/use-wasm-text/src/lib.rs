#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned when WebAssembly text metadata is invalid.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasmTextError {
    /// The supplied value was empty.
    Empty,
    /// The supplied value is not accepted by this crate's conservative rules.
    Invalid,
    /// The supplied S-expression marker is unknown.
    UnknownMarker,
}

impl fmt::Display for WasmTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("WebAssembly text value cannot be empty"),
            Self::Invalid => formatter.write_str("invalid WebAssembly text value"),
            Self::UnknownMarker => formatter.write_str("unknown WebAssembly S-expression marker"),
        }
    }
}

impl Error for WasmTextError {}

fn validate_text_name(value: &str) -> Result<&str, WasmTextError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(WasmTextError::Empty);
    }
    if trimmed
        .chars()
        .any(|character| character.is_control() || character.is_whitespace())
    {
        return Err(WasmTextError::Invalid);
    }
    Ok(trimmed)
}

/// WAT identifier such as '$name'.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WatIdentifier(String);

impl WatIdentifier {
    /// Creates a validated WAT identifier.
    pub fn new(value: impl AsRef<str>) -> Result<Self, WasmTextError> {
        let trimmed = validate_text_name(value.as_ref())?;
        if !trimmed.starts_with('$') || trimmed.len() == 1 {
            return Err(WasmTextError::Invalid);
        }
        Ok(Self(trimmed.to_owned()))
    }

    /// Returns the stored identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the wrapper and returns the stored identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for WatIdentifier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for WatIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for WatIdentifier {
    type Err = WasmTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for WatIdentifier {
    type Error = WasmTextError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// WAT module name metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TextModuleName(String);

impl TextModuleName {
    /// Creates a validated text module name.
    pub fn new(value: impl AsRef<str>) -> Result<Self, WasmTextError> {
        validate_text_name(value.as_ref()).map(|value| Self(value.to_owned()))
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

impl AsRef<str> for TextModuleName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for TextModuleName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for TextModuleName {
    type Err = WasmTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for TextModuleName {
    type Error = WasmTextError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Small vocabulary of S-expression markers used by WebAssembly text.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SExpressionMarker {
    /// Module marker.
    #[default]
    Module,
    /// Function marker.
    Func,
    /// Import marker.
    Import,
    /// Export marker.
    Export,
    /// Memory marker.
    Memory,
    /// Table marker.
    Table,
    /// Global marker.
    Global,
    /// Type marker.
    Type,
    /// Component marker.
    Component,
}

impl SExpressionMarker {
    /// Returns the marker label without the opening parenthesis.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Module => "module",
            Self::Func => "func",
            Self::Import => "import",
            Self::Export => "export",
            Self::Memory => "memory",
            Self::Table => "table",
            Self::Global => "global",
            Self::Type => "type",
            Self::Component => "component",
        }
    }
}

impl fmt::Display for SExpressionMarker {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SExpressionMarker {
    type Err = WasmTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim().trim_start_matches('(');
        if trimmed.is_empty() {
            return Err(WasmTextError::Empty);
        }
        match trimmed.to_ascii_lowercase().as_str() {
            "module" => Ok(Self::Module),
            "func" => Ok(Self::Func),
            "import" => Ok(Self::Import),
            "export" => Ok(Self::Export),
            "memory" => Ok(Self::Memory),
            "table" => Ok(Self::Table),
            "global" => Ok(Self::Global),
            "type" => Ok(Self::Type),
            "component" => Ok(Self::Component),
            _ => Err(WasmTextError::UnknownMarker),
        }
    }
}

/// Returns 'true' when the text starts like a WAT module and has balanced parentheses.
#[must_use]
pub fn looks_like_wat_module(input: &str) -> bool {
    let trimmed = input.trim_start();
    trimmed.starts_with("(module") && has_balanced_parentheses_basic(trimmed)
}

/// Performs a small balanced-parentheses check for text metadata.
#[must_use]
pub fn has_balanced_parentheses_basic(input: &str) -> bool {
    let mut depth = 0_u32;
    for character in input.chars() {
        match character {
            '(' => depth = depth.saturating_add(1),
            ')' => {
                if depth == 0 {
                    return false;
                }
                depth -= 1;
            },
            _ => {},
        }
    }
    depth == 0
}

#[cfg(test)]
mod tests {
    use super::{
        SExpressionMarker, TextModuleName, WasmTextError, WatIdentifier,
        has_balanced_parentheses_basic, looks_like_wat_module,
    };

    #[test]
    fn validates_text_names() {
        let identifier = WatIdentifier::new("$run").expect("valid identifier");
        let module = TextModuleName::new("example").expect("valid module name");

        assert_eq!(identifier.as_str(), "$run");
        assert_eq!(module.to_string(), "example");
        assert_eq!(WatIdentifier::new("run"), Err(WasmTextError::Invalid));
    }

    #[test]
    fn parses_markers_and_checks_text_shape() {
        assert_eq!(
            "(module".parse::<SExpressionMarker>(),
            Ok(SExpressionMarker::Module)
        );
        assert!(looks_like_wat_module("(module (func))"));
        assert!(has_balanced_parentheses_basic("(func (result i32))"));
        assert!(!has_balanced_parentheses_basic("(func"));
    }
}
