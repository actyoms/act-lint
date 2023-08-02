use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{Error as DeError, Unexpected, Visitor};

use crate::{Error, ExpressionError};

/// A string expression that doesn't contain `${{`
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct StringOnly(String);

impl StringOnly {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Create a new string expression.
    pub fn new(s: &str) -> Result<Self, Error> {
        if s.contains("${{") {
            Err(Error::Expression(ExpressionError::expecting(
                s,
                "a string without `${{`",
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

impl fmt::Display for StringOnly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct StringOnlyVisitor;

impl<'de> Visitor<'de> for StringOnlyVisitor {
    type Value = StringOnly;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string without \"${{\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        StringOnly::new(v)
            .map_err(|_| E::invalid_value(Unexpected::Str(v), &self))
    }
}

impl<'de> Deserialize<'de> for StringOnly {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(StringOnlyVisitor)
    }
}

/// A macro to create an [StringOnly] expression [Result]
#[macro_export]
macro_rules! string_only {
    ($s:expr) => {
        $crate::expression::string_only::StringOnly::new($s)
    };
}

/// A macro to create an [StringOnly] expression [Result] and unwrap it
#[macro_export]
macro_rules! must_string_only {
    ($s:expr) => {
        $crate::expression::string_only::StringOnly::must_new($s)
    };
}

#[cfg(test)]
mod tests {
    use serde_yaml::{from_str, to_string};

    use super::*;

    #[test]
    fn deserialize_ok() {
        let e = from_str::<StringOnly>("inputs.abc").unwrap();
        assert_eq!(e, StringOnly::must_new("inputs.abc"));
    }

    #[test]
    fn deserialize_err() {
        let err = from_str::<StringOnly>("${{ inputs.ABC }}").unwrap_err();
        assert_eq!(
            err.to_string(),
            "invalid value: string \"${{ inputs.ABC }}\", expected a string without \"${{\""
        );
    }

    #[test]
    fn serialize() {
        let e = to_string(&StringOnly::must_new("inputs.ABC")).unwrap();
        assert_eq!(e, "inputs.ABC\n")
    }

    #[test]
    fn string_only() {
        assert_eq!(
            string_only!("inputs.abc").unwrap(),
            StringOnly::must_new("inputs.abc")
        );
    }

    #[test]
    fn must_string_only() {
        assert_eq!(
            must_string_only!("inputs.abc"),
            StringOnly::must_new("inputs.abc")
        );
    }
}
