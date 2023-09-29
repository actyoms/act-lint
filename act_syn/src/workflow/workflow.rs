use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{Concurrency, Defaults, Env, Expression, Jobs, On, Permissions};

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
    /// A workflow run is made up of one or more jobs.
    /// Jobs run in parallel by default.
    /// To run jobs sequentially,
    /// you can define dependencies on other jobs
    /// using the jobs.<job_id>.needs keyword.\nEach job runs in a fresh instance of the virtual environment
    /// specified by runs-on.\nYou can run an unlimited number of jobs as long
    /// as you are within the workflow usage limits.
    /// For more information,
    /// see <https://help.github.com/en/github/automating-your-workflow-with-github-actions/workflow-syntax-for-github-actions#usage-limits>.
    ///
    /// <https://help.github.com/en/github/automating-your-workflow-with-github-actions/workflow-syntax-for-github-actions#jobs>
    pub jobs: Jobs,

    /// The name of your workflow.
    /// GitHub displays the names of your workflows on your repository's actions page.
    /// If you omit this field, GitHub sets the name to the workflow's filename.
    ///
    /// <https://help.github.com/en/github/automating-your-workflow-with-github-actions/workflow-syntax-for-github-actions#name>
    pub name: String,

    /// The name of the GitHub event that triggers the workflow.
    /// You can provide a single event string, array of events,
    /// array of event types or an event configuration map that schedules a workflow
    /// or restricts the execution of a workflow to specific files, tags, or branch changes.
    /// For a list of available events,
    /// see <https://help.github.com/en/github/automating-your-workflow-with-github-actions/events-that-trigger-workflows>.
    ///
    /// <https://help.github.com/en/github/automating-your-workflow-with-github-actions/workflow-syntax-for-github-actions#on>
    pub on: On,

    pub permissions: Option<Permissions>,

    /// The name for workflow runs generated from the workflow.
    /// GitHub displays the workflow run name in the list of workflow runs on your repository's Actions tab.
    ///
    /// <https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions#run-name>
    pub run_name: Option<Expression>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use serde_yaml::{from_str, to_string};

    use crate::workflow::Workflow;
    use crate::{job, jobs, On};

    #[test]
    fn deserialize() {
        let expected: Workflow = from_str(
            r#"
jobs:
  build: {}
name: CI
on: push
        "#,
        )
        .unwrap();
        let actual = Workflow {
            concurrency: None,
            defaults: None,
            env: None,
            jobs: jobs! {
                build: job!()
            },
            name: "CI".to_string(),
            on: On::from_str("push").unwrap(),
            permissions: None,
            run_name: None,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn serialize() {
        let w = Workflow {
            concurrency: None,
            defaults: None,
            env: None,
            jobs: jobs! {
                test: job!()
            },
            name: "CI".to_string(),
            on: On::from_str("push").unwrap(),
            permissions: None,
            run_name: None,
        };
        assert_eq!(
            to_string(&w).unwrap().trim(),
            r#"
jobs:
  test: {}
name: CI
on: push"#
                .trim(),
        );
    }
}
