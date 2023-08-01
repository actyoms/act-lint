use crate::error::ExpressionError;

/// An error that occurred during parsing of workflow or action definitions.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// An syntax error occurred during parsing string expression.
    Expression(ExpressionError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Expression(ref err) => write!(f, "{}", err),
        }
    }
}

/// A macro to create an [error for an invalid expression](Error::Expression).
#[macro_export]
macro_rules! error_expression {
    ($($arg:tt)+) => {
        $crate::error::Error::Expression($crate::error::ExpressionError::new($($arg)+))
    };
}

#[cfg(test)]
mod tests {
    use crate::error_expression;

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", error_expression!("x")),
            r#"not a valid string expression: "x""#
        );
    }

    #[test]
    fn error_expression() {
        assert_eq!(
            format!("{}", error_expression!("x")),
            r#"not a valid string expression: "x""#
        );
    }
}
