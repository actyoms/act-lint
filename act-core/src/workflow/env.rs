use std::fmt;
use std::str::FromStr;

use serde_yaml::from_str;

use act_derive::UntaggedDeserialize;

#[allow(unused_imports)]
use crate::expression::{IN_STRING_PATTERN, InString};

/// A [InString] expression or map of environment variables
#[derive(Debug, Eq, PartialEq, serde::Serialize, UntaggedDeserialize)]
#[visitor(EnvVisitor)]
#[serde(untagged)]
pub enum Env {
    InStringExpression(InString),
    Map(std::collections::HashMap<String, String>),
}

impl FromStr for Env {
    type Err = serde_yaml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_str::<InString>(s).map(Env::InStringExpression)
    }
}

struct EnvVisitor;

impl<'de> serde::de::Visitor<'de> for EnvVisitor {
    type Value = Env;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "a string matching {IN_STRING_PATTERN} or map...",
        ))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        from_str::<InString>(v)
            .map(Env::InStringExpression)
            .map_err(|_| E::invalid_value(serde::de::Unexpected::Str(v), &self))
    }

    fn visit_map<A>(self, _: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        todo!()
    }
}

// impl<'de> Deserialize<'de> for Env {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//         where
//             D: Deserializer<'de>,
//     {
//         deserializer.deserialize_any(EnvVisitor)
//     }
// }

#[cfg(test)]
mod tests {
    use serde_yaml::{from_str, to_string};

    use crate::must_in_string;

    use super::*;

    #[test]
    fn deserialize_string_ok() {
        let e: Env = from_str("abc=${{ inputs.ABC }}").unwrap();
        assert_eq!(
            e,
            Env::InStringExpression(must_in_string!("abc=${{ inputs.ABC }}"))
        );
    }

    #[test]
    fn deserialize_string_err() {
        let result = from_str::<Env>("x=y").unwrap_err();
        assert_eq!(
            result.to_string(),
            format!("invalid value: string \"x=y\", expected a string matching {IN_STRING_PATTERN} or map...")
        );
    }

    #[test]
    fn serialize_string_ok() {
        let e: String = to_string(&Env::InStringExpression(must_in_string!(
            "abc=${{ inputs.ABC }}"
        )))
        .unwrap()
        .trim()
        .to_string();
        assert_eq!(e, "abc=${{ inputs.ABC }}");
    }
}
