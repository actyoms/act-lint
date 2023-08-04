use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Job {
    // TODO: job spec
}

#[allow(clippy::derivable_impls)]
impl Default for Job {
    fn default() -> Self {
        Self {}
    }
}

#[macro_export]
macro_rules! job {
    () => {
        $crate::job::Job::default()
    };
}
