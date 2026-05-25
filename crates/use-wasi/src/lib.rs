#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned when WASI primitive labels are invalid.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasiError {
    /// The supplied label was empty.
    Empty,
    /// The supplied label contains unsupported characters.
    Invalid,
    /// The supplied label is unknown for the requested enum.
    Unknown,
}

impl fmt::Display for WasiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("WASI label cannot be empty"),
            Self::Invalid => formatter.write_str("invalid WASI label"),
            Self::Unknown => formatter.write_str("unknown WASI label"),
        }
    }
}

impl Error for WasiError {}

fn validate_wasi_label(value: &str) -> Result<&str, WasiError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(WasiError::Empty);
    }
    if trimmed.chars().any(|character| {
        character.is_control()
            || character.is_whitespace()
            || !(character.is_ascii_alphanumeric()
                || matches!(character, '_' | '-' | '.' | ':' | '/'))
    }) {
        return Err(WasiError::Invalid);
    }
    Ok(trimmed)
}

macro_rules! wasi_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates a validated WASI text wrapper.
            pub fn new(value: impl AsRef<str>) -> Result<Self, WasiError> {
                validate_wasi_label(value.as_ref()).map(|value| Self(value.to_owned()))
            }

            /// Returns the stored text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consumes the wrapper and returns the stored text.
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
            type Err = WasiError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = WasiError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

wasi_text_newtype!(WasiCapabilityLabel);
wasi_text_newtype!(WasiInterfaceName);

/// WASI version family.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasiVersion {
    /// WASI Preview 1.
    #[default]
    Preview1,
    /// WASI Preview 2.
    Preview2,
}

impl WasiVersion {
    /// Returns the stable version label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preview1 => "preview1",
            Self::Preview2 => "preview2",
        }
    }
}

impl fmt::Display for WasiVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for WasiVersion {
    type Err = WasiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(WasiError::Empty);
        }
        match trimmed
            .to_ascii_lowercase()
            .replace(['-', '_'], "")
            .as_str()
        {
            "preview1" | "wasip1" => Ok(Self::Preview1),
            "preview2" | "wasip2" => Ok(Self::Preview2),
            _ => Err(WasiError::Unknown),
        }
    }
}

/// WASI execution profile label.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasiProfile {
    /// Command-style world.
    #[default]
    Command,
    /// Reactor-style world.
    Reactor,
    /// Component Model profile.
    Component,
}

impl WasiProfile {
    /// Returns the stable profile label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Command => "command",
            Self::Reactor => "reactor",
            Self::Component => "component",
        }
    }
}

impl fmt::Display for WasiProfile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for WasiProfile {
    type Err = WasiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(WasiError::Empty);
        }
        match trimmed.to_ascii_lowercase().as_str() {
            "command" => Ok(Self::Command),
            "reactor" => Ok(Self::Reactor),
            "component" => Ok(Self::Component),
            _ => Err(WasiError::Unknown),
        }
    }
}

macro_rules! label_enum {
    ($name:ident { $($variant:ident => $label:literal),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum $name {
            $(
                #[doc = concat!("'", $label, "' marker.")]
                $variant,
            )+
        }

        impl $name {
            /// Returns the stable label.
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $label,)+
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

label_enum!(FilesystemPermission {
    Read => "filesystem.read",
    Write => "filesystem.write",
    Create => "filesystem.create",
    Remove => "filesystem.remove",
});

label_enum!(SocketPermission {
    Tcp => "socket.tcp",
    Udp => "socket.udp",
    IpNameLookup => "socket.ip-name-lookup",
});

label_enum!(EnvironmentPermission {
    Args => "environment.args",
    Environ => "environment.environ",
});

label_enum!(ClockCapability {
    Monotonic => "clock.monotonic",
    WallClock => "clock.wall-clock",
});

label_enum!(RandomCapability {
    Insecure => "random.insecure",
    Secure => "random.secure",
});

#[cfg(test)]
mod tests {
    use super::{
        ClockCapability, FilesystemPermission, WasiCapabilityLabel, WasiError, WasiInterfaceName,
        WasiProfile, WasiVersion,
    };

    #[test]
    fn parses_versions_and_profiles() {
        assert_eq!("wasip1".parse::<WasiVersion>(), Ok(WasiVersion::Preview1));
        assert_eq!(
            "component".parse::<WasiProfile>(),
            Ok(WasiProfile::Component)
        );
        assert_eq!(WasiVersion::Preview2.to_string(), "preview2");
    }

    #[test]
    fn validates_labels_and_permissions() {
        let capability = WasiCapabilityLabel::new("filesystem.read").expect("valid capability");
        let interface = WasiInterfaceName::new("wasi:filesystem/types").expect("valid interface");

        assert_eq!(capability.as_str(), "filesystem.read");
        assert_eq!(interface.as_str(), "wasi:filesystem/types");
        assert_eq!(
            WasiCapabilityLabel::new("bad label"),
            Err(WasiError::Invalid)
        );
        assert_eq!(FilesystemPermission::Read.to_string(), "filesystem.read");
        assert_eq!(ClockCapability::WallClock.to_string(), "clock.wall-clock");
    }
}
