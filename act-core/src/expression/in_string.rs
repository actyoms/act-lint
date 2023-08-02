use std::fmt;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{Error as DeError, Unexpected, Visitor};

use crate::{Error, ExpressionError};

pub static IN_STRING_EXPRESSION_PATTERN: &str = r#"^.*\$\{\{(.|[\r\n])*\}\}.*$"#;

lazy_static! {
    static ref RE: Regex = Regex::new(IN_STRING_EXPRESSION_PATTERN).unwrap();
}

/// a string that should match `^.*\$\{\{(.|[\r\n])*\}\}.*$`
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct InString(String);

impl InString {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Create a new string expression.
    pub fn new(s: &str) -> Result<Self, crate::Error> {
        if !RE.is_match(s) {
            Err(Error::Expression(ExpressionError::expecting(
                s,
                &format!("a string matching {IN_STRING_EXPRESSION_PATTERN}"),
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

impl FromStr for InString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for InString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A macro to create an [InString] expression [Result]
#[macro_export]
macro_rules! in_string {
    ($s:expr) => {
        $crate::expression::in_string::InString::new($s)
    };
}

/// A macro to create an [InString] expression [Result] and unwrap it
#[macro_export]
macro_rules! must_in_string {
    ($s:expr) => {
        $crate::expression::in_string::InString::must_new($s)
    };
}

struct InStringVisitor;

impl<'de> Visitor<'de> for InStringVisitor {
    type Value = InString;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "a string matching {}",
            IN_STRING_EXPRESSION_PATTERN
        ))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        InString::new(v)
            .map_err(|_| E::invalid_value(Unexpected::Str(v), &self))
    }
}

impl<'de> Deserialize<'de> for InString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(InStringVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml::{from_str, to_string};

    use super::*;

    #[test]
    fn deserialize_ok() {
        let e = from_str::<InString>("abc=${{ inputs.abc }}").unwrap();
        assert_eq!(e, InString::must_new("abc=${{ inputs.abc }}"));
    }

    #[test]
    fn deserialize_err() {
        let err = from_str::<InString>("inputs.ABC").unwrap_err();
        assert_eq!(
            err.to_string(),
            format!("invalid value: string \"inputs.ABC\", expected a string matching {IN_STRING_EXPRESSION_PATTERN}")
        );
    }

    #[test]
    fn serialize() {
        let e = to_string(&InString::must_new("abc=${{ inputs.abc }}")).unwrap();
        assert_eq!(e, "abc=${{ inputs.abc }}\n")
    }

    #[test]
    fn in_string() {
        assert_eq!(
            in_string!("abc=${{ inputs.abc }}").unwrap(),
            InString::must_new("abc=${{ inputs.abc }}")
        );
    }

    #[test]
    fn must_in_string() {
        assert_eq!(
            must_in_string!("abc=${{ inputs.abc }}"),
            InString::must_new("abc=${{ inputs.abc }}")
        );
    }
}
