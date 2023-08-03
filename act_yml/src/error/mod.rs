pub use expression::ExpressionError;

pub mod expression;

// pub type Result<T> = std::result::Result<T, Error>;

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
macro_rules! expression_error {
    ($e:expr) => {
        $crate::error::Error::Expression($crate::error::ExpressionError::new($e))
    };
    ($e:expr, $expecting:expr) => {
        $crate::error::Error::Expression($crate::error::ExpressionError::expecting($e, $expecting))
    };
}

#[cfg(test)]
mod tests {
    use crate::expression_error;

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", expression_error!("x")),
            r#"not a valid string expression: "x""#
        );
    }

    #[test]
    fn expression_error() {
        assert_eq!(
            format!("{}", expression_error!("x")),
            r#"not a valid string expression: "x""#
        );
    }
}
