use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{Error, Visitor};

use crate::error_expression;

pub static VALID_EXPRESSION_PATTERN: &str = r#"^.*\$\{\{(.|[\r\n])*\}\}.*$"#;

lazy_static! {
    static ref RE: Regex = Regex::new(VALID_EXPRESSION_PATTERN).unwrap();
}

/// a string that should match `^.*\$\{\{(.|[\r\n])*\}\}.*$`
#[derive(Debug, Eq, PartialEq)]
pub struct ExpressionInString(String);

impl ExpressionInString {
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn new(s: &str) -> Result<Self, crate::Error> {
        if RE.is_match(s) {
            Ok(ExpressionInString(s.to_string()))
        } else {
            Err(error_expression!(s))
        }
    }
}

impl fmt::Display for ExpressionInString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[macro_export]
macro_rules! expression_in_string {
    ($($arg:tt)+) => {
        if RE.is_match($($arg)+) {
            Ok(ExpressionInString($($arg)+.to_string()))
        } else {
            Err(error_expression!($($arg)+))
        }
    };
}

struct ExpressionInStringVisitor;

impl<'de> Visitor<'de> for ExpressionInStringVisitor {
    type Value = ExpressionInString;

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
        if RE.is_match(v) {
            Ok(ExpressionInString(v.to_string()))
        } else {
            Err(Error::invalid_value(serde::de::Unexpected::Str(v), &self))
        }
    }
}

impl<'de> Deserialize<'de> for ExpressionInString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ExpressionInStringVisitor)
    }
}

impl Serialize for ExpressionInString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        if RE.is_match(&self.0) {
            serializer.serialize_str(&self.0)
        } else {
            Err(serde::ser::Error::custom(format!(
                "invalid value: Expression({:?}), expected a string matching {}",
                self.0, VALID_EXPRESSION_PATTERN
            )))
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
            serde_yaml::to_string(&ExpressionInString("x=${{ x }}".to_string())).unwrap(),
            "x=${{ x }}\n"
        );
    }

    #[test]
    fn serialize_err() {
        assert_eq!(
            serde_yaml::to_string(&ExpressionInString("abc".to_string())).unwrap_err().to_string(),
            "invalid value: Expression(\"abc\"), expected a string matching ^.*\\$\\{\\{(.|[\\r\\n])*\\}\\}.*$"
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            from_str::<ExpressionInString>(r#"${{ x }}"#).unwrap(),
            ExpressionInString("${{ x }}".to_string())
        );
    }

    #[test]
    fn deserialize_err() {
        assert_eq!(
            from_str::<ExpressionInString>("abc").unwrap_err().to_string(),
            "invalid value: string \"abc\", expected a string matching ^.*\\$\\{\\{(.|[\\r\\n])*\\}\\}.*$"
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", ExpressionInString("${{ x }}".to_string())),
            r#"${{ x }}"#
        );
    }
}
