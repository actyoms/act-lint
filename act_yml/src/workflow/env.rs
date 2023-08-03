use std::str::FromStr;

use serde_yaml::from_str;

use act_derive::Deserialize;
use act_trait::visit::VisitMap;
use act_trait::Expecting;

#[allow(unused_imports)]
use crate::expression::{InString, IN_STRING_PATTERN};
use crate::StrNumBool;

/// A [InString] expression or map of environment variables
///
/// To set custom environment variables, you need to specify the variables in the workflow file.
/// You can define environment variables for a step, job,
/// or entire workflow using the jobs.<job_id>.steps[*].env, jobs.<job_id>.env, and env keywords.
/// For more information,
/// see <https://docs.github.com/en/actions/learn-github-actions/workflow-syntax-for-github-actions#jobsjob_idstepsenv>
#[derive(Debug, PartialEq, serde::Serialize, Deserialize)]
#[act(visit = ["str", "map"], expecting = "a string matching {IN_STRING_PATTERN} or map...")]
#[serde(untagged)]
pub enum Env {
    InStringExpression(InString),
    Map(std::collections::HashMap<String, StrNumBool>),
}

impl VisitMap<Env> for Env {
    fn visit_map<'a, A>(map: A) -> Result<Env, A::Error>
    where
        A: serde::de::MapAccess<'a>,
    {
        let mut map = map;
        let mut result = std::collections::HashMap::new();
        while let Some((key, value)) = map.next_entry::<String, StrNumBool>()? {
            result.insert(key, value);
        }
        Ok(Env::Map(result))
    }
}

impl FromStr for Env {
    type Err = serde_yaml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_str::<InString>(s).map(Env::InStringExpression)
    }
}

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
    fn deserialize_map_ok<'a>() {
        let e: Env = from_str(
            r#"
        n: 123
        f: 123.456
        b: true
        s: abc
        "#,
        )
        .unwrap();
        let mut map = std::collections::HashMap::new();
        map.insert("n".to_string(), StrNumBool::Int(123));
        map.insert("f".to_string(), StrNumBool::Float(123.456));
        map.insert("b".to_string(), StrNumBool::Bool(true));
        map.insert("s".to_string(), StrNumBool::String("abc".to_string()));
        assert_eq!(e, Env::Map(map));
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
    fn deserialize_map_err() {
        let result = from_str::<Env>(
            r#"
        x:
        - y
        "#,
        )
        .unwrap_err();
        assert_eq!(
            result.to_string(),
            format!("expected one of string, number or boolean at line 2 column 9")
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
