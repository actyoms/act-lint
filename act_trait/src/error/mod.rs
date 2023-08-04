pub use expectation::ExpectationError;
pub use expression::ExpressionError;
pub use value::ValueError;

pub mod expectation;
pub mod expression;
mod value;

// pub type Result<T> = std::result::Result<T, Error>;

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// An syntax error occurred during parsing string expression.
    Expression(ExpressionError),
    Value(ValueError),
}

impl From<ExpressionError> for Error {
    fn from(err: ExpressionError) -> Self {
        Error::Expression(err)
    }
}

impl From<ValueError> for Error {
    fn from(err: ValueError) -> Self {
        Error::Value(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Expression(ref err) => write!(f, "{:?}", err),
            Error::Value(ref err) => write!(f, "{:?}", err),
        }
    }
}
