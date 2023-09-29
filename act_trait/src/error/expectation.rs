use std::fmt;

/// An error that occurs when parsing a value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectationError {
    prefix: String,
    /// The string/ expression that failed to parse.
    actual: String,
    expecting: Option<String>,
}

impl ExpectationError {
    /// Create a new value error for the given string.
    pub fn new(actual: &str) -> Self {
        Self::with_prefix("not a valid value", actual, None)
    }

    /// Create a new value error for the given string with exceptions.
    pub fn expecting(actual: &str, expecting: &str) -> Self {
        Self::with_prefix("not a valid value", actual, Some(expecting.to_string()))
    }

    /// Create a error with prefix, actual and expecting.
    pub fn with_prefix(prefix: &str, actual: &str, expecting: Option<String>) -> Self {
        Self {
            prefix: prefix.to_string(),
            actual: actual.to_string(),
            expecting,
        }
    }
}

impl fmt::Display for ExpectationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(expecting) = &self.expecting {
            write!(
                f,
                r#"{}: "{}", expecting {}"#,
                self.prefix, self.actual, expecting
            )
        } else {
            write!(f, r#"{}: "{}""#, self.prefix, self.actual)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", ExpectationError::new("x")),
            r#"not a valid value: "x""#
        );
    }

    #[test]
    fn display_with_expecting() {
        assert_eq!(
            format!(
                "{}",
                ExpectationError {
                    prefix: "not a valid value type".to_string(),
                    actual: "x".to_string(),
                    expecting: Some("a string matching xyz".to_string()),
                }
            ),
            r#"not a valid value type: "x", expecting a string matching xyz"#
        );
    }
}
