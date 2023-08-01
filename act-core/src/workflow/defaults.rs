use std::fmt;

use serde::de::{MapAccess, Visitor};
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_valid::Validate;
use serde_with::skip_serializing_none;

use crate::workflow::Run;

/// A map of default settings that will apply to all jobs in the workflow
#[skip_serializing_none]
#[derive(Debug, Eq, PartialEq, Serialize, Validate)]
pub struct Defaults {
    /// The default shell and working-directory for steps.
    #[validate(
        min_properties = 1,
        message = "at least one of `shell` or `working-directory` must be specified"
    )]
    pub run: Run,
}

struct DefaultsVisitor;

impl<'de> Visitor<'de> for DefaultsVisitor {
    type Value = Defaults;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map with a `run` key")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<String>()? {
            return match key.as_str() {
                "run" => map
                    .next_value::<Run>()
                    .map(|run| Defaults { run })
                    .and_then(|d| d.validate().map(|_| d).map_err(de::Error::custom)),
                _ => Err(de::Error::unknown_field(&key, &["run"])),
            };
        }
        Err(de::Error::missing_field("run"))
    }
}

impl<'de> Deserialize<'de> for Defaults {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(DefaultsVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_valid::json::json;
    use serde_valid::yaml::FromYamlValue;
    use serde_yaml::from_str;

    use super::*;

    #[test]
    fn deserialize_ok() {
        let s: Defaults = from_str(
            r#"
        run:
          working-directory: /home/runner/work/my-repo/my-repo
          shell: bash
        "#,
        )
        .unwrap();
        assert_eq!(
            s,
            Defaults {
                run: Run {
                    working_directory: Some("/home/runner/work/my-repo/my-repo".to_string()),
                    shell: Some("bash".to_string()),
                },
            }
        );
    }

    #[test]
    fn deserialize_type_err() {
        let err = from_str::<Defaults>("true").unwrap_err();
        assert_eq!(
            err.to_string(),
            "invalid type: boolean `true`, expected a map with a `run` key"
        );
    }

    #[test]
    fn deserialize_missing_err() {
        let err = from_str::<Defaults>("").unwrap_err();
        assert_eq!(err.to_string(), "missing field `run`");
    }

    #[test]
    fn deserialize_empty_run_err() {
        let err = Defaults::from_yaml_value(from_str("run:").unwrap()).unwrap_err();
        assert_eq!(
            err.to_string(),
            json!({
                "errors": [],
                "properties": {
                    "run": {
                        "errors": [
                            "at least one of `shell` or `working-directory` must be specified",
                        ],
                    }
                }
            })
            .to_string()
        );
    }
}
