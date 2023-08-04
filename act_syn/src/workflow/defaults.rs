use act_derive::Deserialize;
use serde::de::MapAccess;
use serde_valid::Validate;
use serde_with::skip_serializing_none;

use act_trait::FromMap;

use crate::Run;

/// A map of default settings that will apply to all jobs in the workflow
#[skip_serializing_none]
#[derive(Debug, Eq, PartialEq, serde::Serialize, Validate, Deserialize)]
#[act(visit = ["map"], expecting = "a map with a `run` key")]
pub struct Defaults {
    /// The default shell and working-directory for steps.
    #[validate(
        min_properties = 1,
        message = "at least one of `shell` or `working-directory` must be specified"
    )]
    pub run: Run,
}

impl FromMap for Defaults {
    fn from_map<'a, A>(mut map: A) -> Result<Defaults, A::Error>
    where
        A: MapAccess<'a>,
    {
        let key = map.next_key::<String>()?;
        if let Some(key) = key {
            match key.as_str() {
                "run" => map
                    .next_value::<Run>()
                    .map(|run| Defaults { run })
                    .and_then(|d| d.validate().map(|_| d).map_err(serde::de::Error::custom)),
                _ => Err(serde::de::Error::unknown_field(&key, &["run"])),
            }
        } else {
            Err(serde::de::Error::missing_field("run"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Defaults, Run};
    use serde_valid::json::json;
    use serde_valid::yaml::FromYamlValue;
    use serde_yaml::from_str;

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
        assert_eq!(
            err.to_string(),
            r#"invalid type: Option value, expected a map with a `run` key"#
        );
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
