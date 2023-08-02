use std::fmt;

/// An error that occurs when parsing a expression string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionError {
    /// The expression that failed to parse.
    expression: String,
    expecting: Option<String>,
}

impl ExpressionError {
    /// Create a new expression error for the given expression.
    pub fn new(expression: &str) -> Self {
        Self {
            expression: expression.to_string(),
            expecting: None,
        }
    }

    /// Create a new expression error for the given expression with expecting.
    pub fn expecting(expression: &str, expecting: &str) -> Self {
        Self {
            expression: expression.to_string(),
            expecting: Some(expecting.to_string()),
        }
    }
}

impl fmt::Display for ExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(expecting) = &self.expecting {
            write!(
                f,
                r#"not a valid string expression: "{}", expecting {}"#,
                self.expression, expecting
            )
        } else {
            write!(f, r#"not a valid string expression: "{}""#, self.expression)
        }
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

    #[test]
    fn display_with_expecting() {
        assert_eq!(
            format!(
                "{}",
                ExpressionError {
                    expression: "x".to_string(),
                    expecting: Some("a string matching xyz".to_string()),
                }
            ),
            r#"not a valid string expression: "x", expecting a string matching xyz"#
        );
    }
}
