use std::str::FromStr;

use act_derive::Deserialize;
use act_trait::from::{FromBool, FromNumber};
use act_trait::Error;

#[derive(Debug, PartialEq, serde::Serialize, Deserialize)]
#[act(visit = ["str", "bool", "number"], expecting = "expected one of string, number or boolean")]
pub enum StrNumBool {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

impl FromStr for StrNumBool {
    type Err = serde_yaml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StrNumBool::String(s.to_string()))
    }
}

impl FromBool for StrNumBool {
    fn from_bool(v: bool) -> Result<Self, Error> {
        Ok(StrNumBool::Bool(v))
    }
}

impl FromNumber for StrNumBool {
    fn from_f64(v: f64) -> Result<Self, Error> {
        Ok(StrNumBool::Float(v))
    }
    fn from_i64(v: i64) -> Result<Self, Error> {
        Ok(StrNumBool::Int(v))
    }
    fn from_u64(v: u64) -> Result<Self, Error> {
        Ok(StrNumBool::Int(v as i64))
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml::from_str;

    use super::*;

    #[test]
    fn deserialize_int_ok() {
        let e: StrNumBool = from_str("123").unwrap();
        assert_eq!(e, StrNumBool::Int(123));
    }

    #[test]
    fn deserialize_float_ok() {
        let e: StrNumBool = from_str("123.456").unwrap();
        assert_eq!(e, StrNumBool::Float(123.456));
    }

    #[test]
    fn deserialize_bool_ok() {
        let e: StrNumBool = from_str("true").unwrap();
        assert_eq!(e, StrNumBool::Bool(true));
    }

    #[test]
    fn deserialize_string_ok() {
        let e: StrNumBool = from_str("abc").unwrap();
        assert_eq!(e, StrNumBool::String("abc".to_string()));
    }

    #[test]
    fn deserialize_err() {
        let err = from_str::<StrNumBool>("null").unwrap_err();
        assert_eq!(
            err.to_string(),
            "invalid type: unit value, expected expected one of string, number or boolean"
        );
    }
}
