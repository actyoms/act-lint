use crate::error::expectation::ExpectationError;

/// An error that occurs when parsing a expression string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueError {
    /// The expression that failed to parse.
    value: ExpectationError,
}

impl ValueError {
    /// Create a new expression error for the given string.
    pub fn new(actual: &str) -> Self {
        Self {
            value: ExpectationError::new(actual),
        }
    }

    /// Create a new expression error for the given string with exceptions.
    pub fn expecting(actual: &str, expecting: &str) -> Self {
        Self {
            value: ExpectationError::expecting(actual, expecting),
        }
    }
}
