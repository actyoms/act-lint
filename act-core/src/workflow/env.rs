use crate::{error_expression, Error};
use lazy_static::lazy_static;
use regex::Regex;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^.*\$\{\{(.|[\r\n])*}}.*$").unwrap();
}

/// A EnvStringExpression or map of environment variables
#[derive(Debug, Eq, PartialEq)]
pub enum Env {
    String(String),
}

impl Env {
    /// Returns true if the string is a valid expression
    pub fn string_expression(s: &str) -> Result<Self, Error> {
        if RE.is_match(s) {
            Ok(Env::String(s.to_string()))
        } else {
            Err(error_expression!(s))
        }
    }
}

struct EnvVisitor;

impl<'de> Visitor<'de> for EnvVisitor {
    type Value = Env;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("stringExpression or map of environment variables")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let val = Env::string_expression(v);
        if val.is_ok() {
            Ok(val.unwrap())
        } else {
            Err(val.map_err(serde::de::Error::custom).unwrap_err())
        }
    }
}

impl<'de> Deserialize<'de> for Env {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(EnvVisitor)
    }
}

impl Serialize for Env {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Env::String(s) => {
                if RE.is_match(s) {
                    serializer.serialize_str(s)
                } else {
                    Err(serde::ser::Error::custom(error_expression!(s)))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error_expression;
    use serde_yaml::{from_str, to_string};

    #[test]
    fn deserialize_string_ok() {
        let e: Env = from_str("abc=${{ inputs.ABC }}").unwrap();
        assert_eq!(e, Env::String("abc=${{ inputs.ABC }}".to_string()));
    }

    #[test]
    fn deserialize_string_err() {
        let result = from_str::<Env>("abc=inputs.ABC").unwrap_err();
        let x = error_expression!("abc=inputs.ABC");
        assert_eq!(result.to_string(), x.to_string());
    }

    #[test]
    fn serialize_string_ok() {
        let e: String = to_string(&Env::String("abc=${{ inputs.ABC }}".to_string()))
            .unwrap()
            .trim()
            .to_string();
        assert_eq!(e, "abc=${{ inputs.ABC }}");
    }

    #[test]
    fn serialize_string_err() {
        let result = to_string(&Env::String("abc=inputs.ABC".to_string())).unwrap_err();
        assert_eq!(
            result.to_string(),
            error_expression!("abc=inputs.ABC").to_string()
        );
    }
}
