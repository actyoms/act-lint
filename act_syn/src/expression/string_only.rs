use std::fmt;
use std::str::FromStr;

use serde::Serialize;

use act_derive::Deserialize;

/// A string expression that doesn't contain `${{`
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[act(visit = ["str"], expecting = "a string without \"${{{{\"")]
pub struct StringOnly(String);

impl StringOnly {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Create a new string expression.
    pub fn new(s: &str) -> Result<Self, act_trait::Error> {
        if s.contains("${{") {
            Err(act_trait::Error::Expression(
                act_trait::ExpressionError::expecting(s, "a string without `${{`"),
            ))
        } else {
            Ok(Self(s.to_string()))
        }
    }
}

impl fmt::Display for StringOnly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for StringOnly {
    type Err = act_trait::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml::{from_str, to_string};

    use super::*;

    #[test]
    fn deserialize_ok() {
        let e = from_str::<StringOnly>("inputs.abc").unwrap();
        assert_eq!(e, StringOnly::new("inputs.abc").unwrap());
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
        let e = to_string(&StringOnly::new("inputs.ABC").unwrap()).unwrap();
        assert_eq!(e, "inputs.ABC\n")
    }
}
