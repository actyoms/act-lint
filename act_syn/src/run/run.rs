use serde::{Deserialize, Serialize};
use serde_valid::{MinPropertiesError, Validate, ValidateMinProperties};
use serde_with::skip_serializing_none;

/// A map of default settings that will apply to all jobs in the workflow
#[skip_serializing_none]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(
    expecting = "a map with at least `shell` or `working-directory` key",
    deny_unknown_fields
)]
#[derive(Validate)]
pub struct Run {
    /// Working directory for steps and the default location for actions that don't specify a directory.
    #[serde(rename = "working-directory")]
    pub working_directory: Option<String>,

    /// The shell to use with the run key. The default shell is bash on Linux and macOS and cmd on Windows.
    pub shell: Option<String>,
}

impl ValidateMinProperties for Run {
    fn validate_min_properties(&self, _: usize) -> Result<(), MinPropertiesError> {
        if self.working_directory.is_some() || self.shell.is_some() {
            Ok(())
        } else {
            Err(MinPropertiesError::new(1usize))
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_valid::validation::ToDefaultMessage;
    use serde_yaml::from_str;

    use super::*;

    #[test]
    fn deserialize_ok() {
        let s: Run = from_str(
            r#"
        working-directory: /home/runner/work/my-repo/my-repo
        shell: bash
        "#,
        )
        .unwrap();
        assert_eq!(
            s,
            Run {
                working_directory: Some("/home/runner/work/my-repo/my-repo".to_string()),
                shell: Some("bash".to_string()),
            }
        );
    }

    #[test]
    fn deserialize_err() {
        let err = from_str::<Run>("true").unwrap_err();
        assert_eq!(err.to_string(), "invalid type: boolean `true`, expected a map with at least `shell` or `working-directory` key");
    }

    #[test]
    fn serialize() {
        let s = Run {
            working_directory: Some("/home/runner/work/my-repo/my-repo".to_string()),
            shell: Some("bash".to_string()),
        };
        assert_eq!(
            serde_yaml::to_string(&s).unwrap(),
            "working-directory: /home/runner/work/my-repo/my-repo\nshell: bash\n"
        );
    }

    #[test]
    fn validate_err() {
        let s: Run = Run {
            working_directory: None,
            shell: None,
        };
        assert_eq!(
            s.validate_min_properties(1)
                .unwrap_err()
                .to_default_message(),
            r#"The size of the properties must be `>= 1`."#
        );
    }
}
