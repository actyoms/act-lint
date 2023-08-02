use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{Error as DeError, Unexpected, Visitor};

use crate::{Error, ExpressionError};

pub static IN_SYNTAX_EXPRESSION_PATTERN: &str = r#"^\$\{\{(.|[\r\n])*\}\}$"#;

lazy_static! {
    static ref RE: Regex = Regex::new(IN_SYNTAX_EXPRESSION_PATTERN).unwrap();
}

/// a string that should match `^\$\{\{(.|[\r\n])*\}\}$`
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct InSyntax(String);

impl InSyntax {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Create a new string expression.
    pub fn new(s: &str) -> Result<Self, crate::Error> {
        if !RE.is_match(s) {
            Err(Error::Expression(ExpressionError::expecting(
                s,
                &format!("a string matching {IN_SYNTAX_EXPRESSION_PATTERN}"),
            )))
        } else {
            Ok(Self(s.to_string()))
        }
    }

    /// Must create a new string expression.
    pub fn must_new(s: &str) -> Self {
        Self::new(s).unwrap()
    }
}

impl fmt::Display for InSyntax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A macro to create an [InSyntax] expression [Result]
#[macro_export]
macro_rules! in_syntax {
    ($s:expr) => {
        $crate::expression::in_syntax::InSyntax::new($s)
    };
}

/// A macro to create an [InSyntax] expression [Result] and unwrap it
#[macro_export]
macro_rules! must_in_syntax {
    ($s:expr) => {
        $crate::expression::in_syntax::InSyntax::must_new($s)
    };
}

struct InSyntaxVisitor;

impl<'de> Visitor<'de> for InSyntaxVisitor {
    type Value = InSyntax;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "a string matching {}",
            IN_SYNTAX_EXPRESSION_PATTERN
        ))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        InSyntax::new(v)
            .map_err(|_| E::invalid_value(Unexpected::Str(v), &self))
    }
}

impl<'de> Deserialize<'de> for InSyntax {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(InSyntaxVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml::{from_str, to_string};

    use super::*;

    #[test]
    fn deserialize_ok() {
        let e = from_str::<InSyntax>("${{ inputs.abc }}").unwrap();
        assert_eq!(e, InSyntax::must_new("${{ inputs.abc }}"));
    }

    #[test]
    fn deserialize_err() {
        let err = from_str::<InSyntax>("inputs.ABC").unwrap_err();
        assert_eq!(
            err.to_string(),
            format!("invalid value: string \"inputs.ABC\", expected a string matching {IN_SYNTAX_EXPRESSION_PATTERN}")
        );
    }

    #[test]
    fn serialize() {
        let e = to_string(&InSyntax::must_new("${{ inputs.abc }}")).unwrap();
        assert_eq!(e, "${{ inputs.abc }}\n")
    }

    #[test]
    fn in_syntax() {
        assert_eq!(
            in_syntax!("${{ inputs.abc }}").unwrap(),
            InSyntax::must_new("${{ inputs.abc }}")
        );
    }

    #[test]
    fn must_in_syntax() {
        assert_eq!(
            must_in_syntax!("${{ inputs.abc }}"),
            InSyntax::must_new("${{ inputs.abc }}")
        );
    }
}
