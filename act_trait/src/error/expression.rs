use std::fmt;

use crate::error::expectation::ExpectationError;

/// An error that occurs when parsing a expression string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionError {
    /// The expression that failed to parse.
    expectation: ExpectationError,
}

impl ExpressionError {
    /// Create a new expression error for the given string.
    pub fn new(actual: &str) -> Self {
        Self {
            expectation: ExpectationError::with_prefix("not a valid expression", actual, None),
        }
    }

    /// Create a new expression error for the given string with exceptions.
    pub fn expecting(actual: &str, expecting: &str) -> Self {
        Self {
            expectation: ExpectationError::with_prefix(
                "not a valid expression",
                actual,
                Some(expecting.to_string()),
            ),
        }
    }
}

impl fmt::Display for ExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expectation)
    }
}
