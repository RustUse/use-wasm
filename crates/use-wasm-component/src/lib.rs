#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned when Component Model names are invalid.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ComponentNameError {
    /// The supplied value was empty.
    Empty,
    /// The supplied value does not match this crate's conservative name rules.
    Invalid,
    /// The supplied item kind label was not recognized.
    UnknownKind,
}

impl fmt::Display for ComponentNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("Component Model name cannot be empty"),
            Self::Invalid => formatter.write_str("invalid Component Model name"),
            Self::UnknownKind => formatter.write_str("unknown Component Model item kind"),
        }
    }
}

impl Error for ComponentNameError {}

fn is_component_segment(value: &str) -> bool {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && characters
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
}

fn validate_component_name(value: &str) -> Result<&str, ComponentNameError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(ComponentNameError::Empty);
    }
    if is_component_segment(trimmed) {
        Ok(trimmed)
    } else {
        Err(ComponentNameError::Invalid)
    }
}

fn validate_package_reference(value: &str) -> Result<&str, ComponentNameError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(ComponentNameError::Empty);
    }
    let without_version = trimmed.split_once('@').map_or(trimmed, |(name, _)| name);
    let mut parts = without_version.split(':');
    let Some(namespace) = parts.next() else {
        return Err(ComponentNameError::Invalid);
    };
    let Some(name) = parts.next() else {
        return Err(ComponentNameError::Invalid);
    };
    if parts.next().is_some() || !is_component_segment(namespace) || !is_component_segment(name) {
        return Err(ComponentNameError::Invalid);
    }
    Ok(trimmed)
}

macro_rules! component_name_newtype {
    ($name:ident, $validator:path) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates a validated Component Model name wrapper.
            pub fn new(value: impl AsRef<str>) -> Result<Self, ComponentNameError> {
                $validator(value.as_ref()).map(|value| Self(value.to_owned()))
            }

            /// Returns the stored name.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consumes the wrapper and returns the stored name.
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
            type Err = ComponentNameError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = ComponentNameError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

component_name_newtype!(ComponentName, validate_component_name);
component_name_newtype!(WorldName, validate_component_name);
component_name_newtype!(InterfaceName, validate_component_name);
component_name_newtype!(PackageReference, validate_package_reference);

/// Component Model import/export item kind.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ComponentItemKind {
    /// Function item.
    #[default]
    Function,
    /// Type item.
    Type,
    /// Interface item.
    Interface,
    /// Instance item.
    Instance,
    /// Nested component item.
    Component,
    /// Resource item.
    Resource,
    /// Value item.
    Value,
}

impl ComponentItemKind {
    /// Returns the stable kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Function => "function",
            Self::Type => "type",
            Self::Interface => "interface",
            Self::Instance => "instance",
            Self::Component => "component",
            Self::Resource => "resource",
            Self::Value => "value",
        }
    }
}

impl fmt::Display for ComponentItemKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ComponentItemKind {
    type Err = ComponentNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ComponentNameError::Empty);
        }
        match trimmed.to_ascii_lowercase().as_str() {
            "function" | "func" => Ok(Self::Function),
            "type" => Ok(Self::Type),
            "interface" => Ok(Self::Interface),
            "instance" => Ok(Self::Instance),
            "component" => Ok(Self::Component),
            "resource" => Ok(Self::Resource),
            "value" => Ok(Self::Value),
            _ => Err(ComponentNameError::UnknownKind),
        }
    }
}

/// Component import metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ComponentImport {
    name: InterfaceName,
    kind: ComponentItemKind,
}

impl ComponentImport {
    /// Creates component import metadata.
    #[must_use]
    pub const fn new(name: InterfaceName, kind: ComponentItemKind) -> Self {
        Self { name, kind }
    }

    /// Returns the import name.
    #[must_use]
    pub const fn name(&self) -> &InterfaceName {
        &self.name
    }

    /// Returns the import kind.
    #[must_use]
    pub const fn kind(&self) -> ComponentItemKind {
        self.kind
    }
}

/// Component export metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ComponentExport {
    name: InterfaceName,
    kind: ComponentItemKind,
}

impl ComponentExport {
    /// Creates component export metadata.
    #[must_use]
    pub const fn new(name: InterfaceName, kind: ComponentItemKind) -> Self {
        Self { name, kind }
    }

    /// Returns the export name.
    #[must_use]
    pub const fn name(&self) -> &InterfaceName {
        &self.name
    }

    /// Returns the export kind.
    #[must_use]
    pub const fn kind(&self) -> ComponentItemKind {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ComponentImport, ComponentItemKind, ComponentNameError, InterfaceName, PackageReference,
        WorldName,
    };

    #[test]
    fn validates_component_names() {
        let world = WorldName::new("cli").expect("valid world");
        let package = PackageReference::new("wasi:cli@0.2.0").expect("valid package");

        assert_eq!(world.as_str(), "cli");
        assert_eq!(package.as_str(), "wasi:cli@0.2.0");
        assert_eq!(WorldName::new("bad name"), Err(ComponentNameError::Invalid));
    }

    #[test]
    fn parses_item_kinds_and_metadata() {
        let kind = "interface"
            .parse::<ComponentItemKind>()
            .expect("known kind");
        let import = ComponentImport::new(
            InterfaceName::new("filesystem").expect("valid interface"),
            kind,
        );

        assert_eq!(kind.to_string(), "interface");
        assert_eq!(import.name().as_str(), "filesystem");
        assert_eq!(import.kind(), ComponentItemKind::Interface);
    }
}
