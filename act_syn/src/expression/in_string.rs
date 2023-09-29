use std::fmt;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;

use act_derive::Deserialize;

pub static IN_STRING_EXPRESSION_PATTERN: &str = r#"^.*\$\{\{(.|[\r\n])*\}\}.*$"#;

lazy_static! {
    static ref RE: Regex = Regex::new(IN_STRING_EXPRESSION_PATTERN).unwrap();
}

const _: &str = r#"asd?"#;

/// a string that should match `^.*\$\{\{(.|[\r\n])*\}\}.*$`
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[act(visit = ["str"], expecting = "a string matching {IN_STRING_EXPRESSION_PATTERN}")]
pub struct InString(String);

impl InString {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Create a new string expression.
    pub fn new(s: &str) -> Result<Self, act_trait::Error> {
        if !RE.is_match(s) {
            Err(act_trait::Error::Expression(
                act_trait::ExpressionError::expecting(
                    s,
                    &format!("a string matching {IN_STRING_EXPRESSION_PATTERN}"),
                ),
            ))
        } else {
            Ok(Self(s.to_string()))
        }
    }
}

impl FromStr for InString {
    type Err = act_trait::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for InString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml::{from_str, to_string};

    use super::*;

    #[test]
    fn deserialize_ok() {
        let e = from_str::<InString>("abc=${{ inputs.abc }}").unwrap();
        assert_eq!(e, InString::new("abc=${{ inputs.abc }}").unwrap());
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
        let e = to_string(&InString::new("abc=${{ inputs.abc }}").unwrap()).unwrap();
        assert_eq!(e, "abc=${{ inputs.abc }}\n")
    }

    #[test]
    fn in_string() {
        assert_eq!(
            InString::new("abc=${{ inputs.abc }}").unwrap(),
            InString::new("abc=${{ inputs.abc }}").unwrap()
        );
    }
}
