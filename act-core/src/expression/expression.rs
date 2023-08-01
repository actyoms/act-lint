use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

use crate::error_expression;

pub static VALID_EXPRESSION_PATTERN: &str = r#"^\$\{\{(.|[\r\n])*\}\}$"#;

lazy_static! {
    static ref RE: Regex = Regex::new(VALID_EXPRESSION_PATTERN).unwrap();
}

/// a string that should match `^\$\{\{(.|[\r\n])*\}\}$`
#[derive(Debug, Eq, PartialEq)]
pub struct Expression(String);

impl Expression {
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn new(s: &str) -> Result<Self, crate::Error> {
        if RE.is_match(s) {
            Err(error_expression!(s))
        } else {
            Ok(Expression(s.to_string()))
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct ExpressionVisitor;

impl<'de> Visitor<'de> for ExpressionVisitor {
    type Value = Expression;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "a string matching {}",
            VALID_EXPRESSION_PATTERN
        ))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if !RE.is_match(v) {
            Err(Error::invalid_value(serde::de::Unexpected::Str(v), &self))
        } else {
            Ok(Expression(v.to_string()))
        }
    }
}

impl<'de> Deserialize<'de> for Expression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ExpressionVisitor)
    }
}

impl Serialize for Expression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if !RE.is_match(&self.0) {
            Err(serde::ser::Error::custom(format!(
                "invalid value: Expression({:?}), expected a string matching {}",
                self.0, VALID_EXPRESSION_PATTERN
            )))
        } else {
            serializer.serialize_str(&self.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml::from_str;

    use super::*;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_yaml::to_string(&Expression("${{ x }}".to_string())).unwrap(),
            "${{ x }}\n"
        );
    }

    #[test]
    fn serialize_err() {
        assert_eq!(
            serde_yaml::to_string(&Expression("abc".to_string())).unwrap_err().to_string(),
            "invalid value: Expression(\"abc\"), expected a string matching ^\\$\\{\\{(.|[\\r\\n])*\\}\\}$"
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            from_str::<Expression>(r#"${{ x }}"#).unwrap(),
            Expression("${{ x }}".to_string())
        );
    }

    #[test]
    fn deserialize_err() {
        assert_eq!(
            from_str::<Expression>("abc").unwrap_err().to_string(),
            "invalid value: string \"abc\", expected a string matching ^\\$\\{\\{(.|[\\r\\n])*\\}\\}$"
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", Expression("${{ x }}".to_string())),
            r#"${{ x }}"#
        );
    }
}
