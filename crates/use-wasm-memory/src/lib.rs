#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::error::Error;

/// WebAssembly page size in bytes.
pub const WASM_PAGE_SIZE_BYTES: u64 = 65_536;

/// WebAssembly page size in KiB.
pub const WASM_PAGE_SIZE_KIB: u64 = 64;

/// Error returned when memory limits are invalid.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WasmMemoryError {
    /// The byte count is not a whole number of Wasm pages.
    BytesNotPageAligned,
    /// The page count does not fit in 'u32'.
    PageCountOverflow,
    /// The maximum page count is lower than the minimum page count.
    MaximumLessThanMinimum,
}

impl fmt::Display for WasmMemoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BytesNotPageAligned => {
                formatter.write_str("byte count is not aligned to WebAssembly pages")
            },
            Self::PageCountOverflow => formatter.write_str("WebAssembly page count exceeds u32"),
            Self::MaximumLessThanMinimum => {
                formatter.write_str("maximum memory pages cannot be lower than minimum pages")
            },
        }
    }
}

impl Error for WasmMemoryError {}

/// WebAssembly page count.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WasmPageCount(u32);

impl WasmPageCount {
    /// Creates a page count from a raw page value.
    #[must_use]
    pub const fn new(pages: u32) -> Self {
        Self(pages)
    }

    /// Creates a page count from a byte count.
    pub fn from_bytes(bytes: u64) -> Result<Self, WasmMemoryError> {
        if !bytes.is_multiple_of(WASM_PAGE_SIZE_BYTES) {
            return Err(WasmMemoryError::BytesNotPageAligned);
        }
        let pages = bytes / WASM_PAGE_SIZE_BYTES;
        let pages = u32::try_from(pages).map_err(|_| WasmMemoryError::PageCountOverflow)?;
        Ok(Self(pages))
    }

    /// Returns the raw page count.
    #[must_use]
    pub const fn pages(self) -> u32 {
        self.0
    }

    /// Returns the represented byte count.
    #[must_use]
    pub fn bytes(self) -> u64 {
        u64::from(self.0) * WASM_PAGE_SIZE_BYTES
    }
}

impl fmt::Display for WasmPageCount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} pages", self.pages())
    }
}

/// Minimum linear memory size marker.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MemoryMinimum(WasmPageCount);

impl MemoryMinimum {
    /// Creates a minimum memory marker.
    #[must_use]
    pub const fn new(pages: WasmPageCount) -> Self {
        Self(pages)
    }

    /// Returns the wrapped page count.
    #[must_use]
    pub const fn page_count(self) -> WasmPageCount {
        self.0
    }

    /// Returns the raw page count.
    #[must_use]
    pub const fn pages(self) -> u32 {
        self.0.pages()
    }

    /// Returns the represented byte count.
    #[must_use]
    pub fn bytes(self) -> u64 {
        self.0.bytes()
    }
}

impl From<WasmPageCount> for MemoryMinimum {
    fn from(value: WasmPageCount) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for MemoryMinimum {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "minimum {}", self.page_count())
    }
}

/// Maximum linear memory size marker.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MemoryMaximum(WasmPageCount);

impl MemoryMaximum {
    /// Creates a maximum memory marker.
    #[must_use]
    pub const fn new(pages: WasmPageCount) -> Self {
        Self(pages)
    }

    /// Returns the wrapped page count.
    #[must_use]
    pub const fn page_count(self) -> WasmPageCount {
        self.0
    }

    /// Returns the raw page count.
    #[must_use]
    pub const fn pages(self) -> u32 {
        self.0.pages()
    }

    /// Returns the represented byte count.
    #[must_use]
    pub fn bytes(self) -> u64 {
        self.0.bytes()
    }
}

impl From<WasmPageCount> for MemoryMaximum {
    fn from(value: WasmPageCount) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for MemoryMaximum {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "maximum {}", self.page_count())
    }
}

/// Shared linear memory marker.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SharedMemory {
    /// Memory is not shared.
    #[default]
    Unshared,
    /// Memory is shared.
    Shared,
}

impl SharedMemory {
    /// Creates a shared-memory marker from a boolean.
    #[must_use]
    pub const fn from_bool(shared: bool) -> Self {
        if shared { Self::Shared } else { Self::Unshared }
    }

    /// Returns the stable marker label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unshared => "unshared",
            Self::Shared => "shared",
        }
    }

    /// Returns 'true' when memory is shared.
    #[must_use]
    pub const fn is_shared(self) -> bool {
        matches!(self, Self::Shared)
    }
}

impl From<bool> for SharedMemory {
    fn from(value: bool) -> Self {
        Self::from_bool(value)
    }
}

impl From<SharedMemory> for bool {
    fn from(value: SharedMemory) -> Self {
        value.is_shared()
    }
}

impl fmt::Display for SharedMemory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Linear memory limit metadata.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct MemoryLimits {
    minimum: WasmPageCount,
    maximum: Option<WasmPageCount>,
    shared: SharedMemory,
}

impl MemoryLimits {
    /// Creates memory limits after checking that 'maximum >= minimum' when present.
    pub fn new(
        minimum: WasmPageCount,
        maximum: Option<WasmPageCount>,
    ) -> Result<Self, WasmMemoryError> {
        if let Some(maximum) = maximum
            && maximum < minimum
        {
            return Err(WasmMemoryError::MaximumLessThanMinimum);
        }

        Ok(Self {
            minimum,
            maximum,
            shared: SharedMemory::Unshared,
        })
    }

    /// Marks the memory as shared or unshared.
    #[must_use]
    pub const fn with_shared(mut self, shared: bool) -> Self {
        self.shared = SharedMemory::from_bool(shared);
        self
    }

    /// Marks the memory with an explicit shared-memory marker.
    #[must_use]
    pub const fn with_shared_memory(mut self, shared: SharedMemory) -> Self {
        self.shared = shared;
        self
    }

    /// Returns the minimum page count.
    #[must_use]
    pub const fn minimum(&self) -> WasmPageCount {
        self.minimum
    }

    /// Returns the minimum page count as a raw integer.
    #[must_use]
    pub const fn minimum_pages(&self) -> u32 {
        self.minimum.pages()
    }

    /// Returns the maximum page count when bounded.
    #[must_use]
    pub const fn maximum(&self) -> Option<WasmPageCount> {
        self.maximum
    }

    /// Returns the maximum page count as a raw integer when bounded.
    #[must_use]
    pub const fn maximum_pages(&self) -> Option<u32> {
        match self.maximum {
            Some(maximum) => Some(maximum.pages()),
            None => None,
        }
    }

    /// Returns the shared-memory marker.
    #[must_use]
    pub const fn shared_memory(&self) -> SharedMemory {
        self.shared
    }

    /// Returns 'true' when the memory is marked as shared.
    #[must_use]
    pub const fn is_shared(&self) -> bool {
        self.shared.is_shared()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        MemoryLimits, MemoryMaximum, MemoryMinimum, SharedMemory, WASM_PAGE_SIZE_BYTES,
        WasmMemoryError, WasmPageCount,
    };

    #[test]
    fn converts_pages_and_bytes() {
        let pages = WasmPageCount::from_bytes(WASM_PAGE_SIZE_BYTES * 2).expect("aligned pages");

        assert_eq!(pages.pages(), 2);
        assert_eq!(pages.bytes(), WASM_PAGE_SIZE_BYTES * 2);
        assert_eq!(pages.to_string(), "2 pages");
        assert_eq!(MemoryMinimum::new(pages).pages(), 2);
        assert_eq!(MemoryMaximum::new(pages).bytes(), WASM_PAGE_SIZE_BYTES * 2);
    }

    #[test]
    fn validates_memory_limits() {
        let limits = MemoryLimits::new(WasmPageCount::new(1), Some(WasmPageCount::new(4)))
            .expect("valid limits")
            .with_shared(true);

        assert_eq!(limits.minimum_pages(), 1);
        assert_eq!(limits.maximum_pages(), Some(4));
        assert_eq!(limits.shared_memory(), SharedMemory::Shared);
        assert!(limits.is_shared());
        assert_eq!(SharedMemory::from_bool(false).to_string(), "unshared");
        assert_eq!(
            MemoryLimits::new(WasmPageCount::new(4), Some(WasmPageCount::new(1))),
            Err(WasmMemoryError::MaximumLessThanMinimum)
        );
    }
}
