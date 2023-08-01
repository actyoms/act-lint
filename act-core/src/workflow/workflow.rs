use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::workflow::{Concurrency, Defaults, Env};
use crate::Expression;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    /// Concurrency ensures that only a single job or workflow using the same concurrency group will run at a time.
    /// A concurrency group can be any string or expression. The expression can use any context except for the secrets context.
    /// You can also specify concurrency at the workflow level. When a concurrent job or workflow is queued,
    /// if another job or workflow using the same concurrency group in the repository is in progress,
    /// the queued job or workflow will be pending. Any previously pending job or workflow in the concurrency group will be canceled.
    /// To also cancel any currently running job or workflow in the same concurrency group, specify cancel-in-progress: true.
    ///
    /// <https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions#concurrency>
    pub concurrency: Option<Concurrency>,

    /// "A map of default settings that will apply to all jobs in the workflow
    ///
    /// <https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions#defaults>
    pub defaults: Option<Defaults>,

    /// A map of environment variables that are available to all jobs and steps in the workflow
    ///
    /// <https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions#env>
    pub env: Option<Env>,
    // jobs
    pub name: String,
    // on
    // permissions
    pub run_name: Option<Expression>,
}

#[cfg(test)]
mod tests {
    use serde_yaml::{from_str, to_string};

    use crate::workflow::Workflow;

    #[test]
    fn deserialize() {
        let w: Workflow = from_str(
            r#"
        name: CI
        "#,
        )
        .unwrap();
        assert_eq!(
            w,
            Workflow {
                concurrency: None,
                defaults: None,
                env: None,
                name: "CI".to_string(),
                run_name: None,
            }
        );
    }

    #[test]
    fn serialize() {
        let w = Workflow {
            concurrency: None,
            defaults: None,
            env: None,
            name: "CI".to_string(),
            run_name: None,
        };
        assert_eq!(to_string(&w).unwrap(), "name: CI\n")
    }
}
