#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned when WIT text identifiers are invalid.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WitNameError {
    /// The supplied value was empty.
    Empty,
    /// The supplied value does not match this crate's conservative WIT identifier rules.
    Invalid,
}

impl fmt::Display for WitNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("WIT name cannot be empty"),
            Self::Invalid => formatter.write_str("invalid WIT name"),
        }
    }
}

impl Error for WitNameError {}

fn is_wit_segment(value: &str) -> bool {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && characters
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
}

fn validate_wit_identifier(value: &str) -> Result<&str, WitNameError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(WitNameError::Empty);
    }
    if is_wit_segment(trimmed) {
        Ok(trimmed)
    } else {
        Err(WitNameError::Invalid)
    }
}

fn validate_wit_package_name(value: &str) -> Result<&str, WitNameError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(WitNameError::Empty);
    }
    let package_without_version = trimmed.split_once('@').map_or(trimmed, |(name, _)| name);
    let mut parts = package_without_version.split(':');
    let Some(namespace) = parts.next() else {
        return Err(WitNameError::Invalid);
    };
    let Some(name) = parts.next() else {
        return Err(WitNameError::Invalid);
    };
    if parts.next().is_some() || !is_wit_segment(namespace) || !is_wit_segment(name) {
        return Err(WitNameError::Invalid);
    }
    Ok(trimmed)
}

macro_rules! wit_name_newtype {
    ($name:ident, $validator:path) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates a validated WIT name wrapper.
            pub fn new(value: impl AsRef<str>) -> Result<Self, WitNameError> {
                $validator(value.as_ref()).map(|value| Self(value.to_owned()))
            }

            /// Returns the stored WIT name.
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
            type Err = WitNameError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = WitNameError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

wit_name_newtype!(WitPackageName, validate_wit_package_name);
wit_name_newtype!(WitNamespace, validate_wit_identifier);
wit_name_newtype!(WitInterfaceName, validate_wit_identifier);
wit_name_newtype!(WitWorldName, validate_wit_identifier);
wit_name_newtype!(WitTypeName, validate_wit_identifier);
wit_name_newtype!(WitFunctionName, validate_wit_identifier);
wit_name_newtype!(WitResourceName, validate_wit_identifier);

/// Returns 'true' when a label fits the conservative WIT identifier rule.
#[must_use]
pub fn is_valid_wit_identifier(value: &str) -> bool {
    validate_wit_identifier(value).is_ok()
}

#[cfg(test)]
mod tests {
    use super::{
        WitFunctionName, WitNameError, WitPackageName, WitWorldName, is_valid_wit_identifier,
    };

    #[test]
    fn validates_wit_identifiers() {
        let world = WitWorldName::new("cli").expect("valid world");
        let function = WitFunctionName::new("read-file").expect("valid function");

        assert_eq!(world.as_str(), "cli");
        assert_eq!(function.to_string(), "read-file");
        assert!(is_valid_wit_identifier("resource_name"));
        assert!(!is_valid_wit_identifier("1bad"));
    }

    #[test]
    fn validates_package_names() {
        let package = WitPackageName::new("wasi:filesystem@0.2.0").expect("valid package");

        assert_eq!(package.as_str(), "wasi:filesystem@0.2.0");
        assert_eq!(
            WitPackageName::new("filesystem"),
            Err(WitNameError::Invalid)
        );
    }
}
