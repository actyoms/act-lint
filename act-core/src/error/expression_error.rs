use std::fmt;

/// An error that occurs when parsing a expression string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionError {
    /// The expression that failed to parse.
    expression: String,
}

impl ExpressionError {
    /// Create a new expression error for the given expression.
    pub fn new(expression: &str) -> Self {
        Self {
            expression: expression.to_string(),
        }
    }
}

impl fmt::Display for ExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"not a valid string expression: "{}""#, self.expression)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", ExpressionError::new("x")),
            r#"not a valid string expression: "x""#
        );
    }
}
