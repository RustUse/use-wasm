#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

pub use use_wasm_value::{
    WasmValueType as FunctionValueType, WasmValueTypeError as WasmFunctionError,
};

macro_rules! index_newtype {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(u32);

        impl $name {
            /// Creates an index wrapper.
            #[must_use]
            pub const fn new(index: u32) -> Self {
                Self(index)
            }

            /// Returns the wrapped index.
            #[must_use]
            pub const fn get(self) -> u32 {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "{}", self.get())
            }
        }
    };
}

index_newtype!(FunctionIndex);
index_newtype!(TypeIndex);
index_newtype!(LocalIndex);

/// Function parameter list metadata.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct ParameterList(Vec<FunctionValueType>);

impl ParameterList {
    /// Creates a parameter list from value type labels.
    #[must_use]
    pub fn new(values: Vec<FunctionValueType>) -> Self {
        Self(values)
    }

    /// Returns an empty parameter list.
    #[must_use]
    pub const fn empty() -> Self {
        Self(Vec::new())
    }

    /// Returns the parameter list as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[FunctionValueType] {
        &self.0
    }

    /// Returns the number of parameters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns 'true' when no parameters are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Function result list metadata.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct ResultList(Vec<FunctionValueType>);

impl ResultList {
    /// Creates a result list from value type labels.
    #[must_use]
    pub fn new(values: Vec<FunctionValueType>) -> Self {
        Self(values)
    }

    /// Returns an empty result list.
    #[must_use]
    pub const fn empty() -> Self {
        Self(Vec::new())
    }

    /// Returns the result list as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[FunctionValueType] {
        &self.0
    }
}

/// Function signature metadata.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct FunctionSignature {
    params: ParameterList,
    results: ResultList,
}

impl FunctionSignature {
    /// Creates function signature metadata.
    #[must_use]
    pub const fn new(params: ParameterList, results: ResultList) -> Self {
        Self { params, results }
    }

    /// Returns parameter metadata.
    #[must_use]
    pub const fn params(&self) -> &ParameterList {
        &self.params
    }

    /// Returns result metadata.
    #[must_use]
    pub const fn results(&self) -> &ResultList {
        &self.results
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FunctionIndex, FunctionSignature, FunctionValueType, ParameterList, ResultList,
        WasmFunctionError,
    };

    #[test]
    fn wraps_indexes_and_value_types() {
        let index = FunctionIndex::new(3);
        let value_type = "extern-ref"
            .parse::<FunctionValueType>()
            .expect("known type");

        assert_eq!(index.get(), 3);
        assert_eq!(index.to_string(), "3");
        assert_eq!(value_type, FunctionValueType::ExternRef);
        assert_eq!(
            "".parse::<FunctionValueType>(),
            Err(WasmFunctionError::Empty)
        );
    }

    #[test]
    fn stores_signature_metadata() {
        let signature = FunctionSignature::new(
            ParameterList::new(vec![FunctionValueType::I32, FunctionValueType::I64]),
            ResultList::new(vec![FunctionValueType::I32]),
        );

        assert_eq!(signature.params().len(), 2);
        assert_eq!(signature.results().as_slice(), &[FunctionValueType::I32]);
    }
}
