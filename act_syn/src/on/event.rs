use std::str::FromStr;

use lazy_static::lazy_static;

use act_derive::Deserialize;
use act_trait::{Error, ValueError};

lazy_static! {
    pub static ref EVENTS: Vec<String> = vec![
        "branch_protection_rule".to_string(),
        "check_run".to_string(),
        "check_suite".to_string(),
        "create".to_string(),
        "delete".to_string(),
        "deployment".to_string(),
        "deployment_status".to_string(),
        "discussion".to_string(),
        "discussion_comment".to_string(),
        "fork".to_string(),
        "gollum".to_string(),
        "issue_comment".to_string(),
        "issues".to_string(),
        "label".to_string(),
        "member".to_string(),
        "milestone".to_string(),
        "page_build".to_string(),
        "project".to_string(),
        "project_card".to_string(),
        "project_column".to_string(),
        "public".to_string(),
        "pull_request".to_string(),
        "pull_request_review".to_string(),
        "pull_request_review_comment".to_string(),
        "pull_request_target".to_string(),
        "push".to_string(),
        "registry_package".to_string(),
        "release".to_string(),
        "repository_dispatch".to_string(),
        "status".to_string(),
        "watch".to_string(),
        "workflow_call".to_string(),
        "workflow_dispatch".to_string(),
        "workflow_run".to_string(),
    ];
}

#[derive(Debug, PartialEq, serde::Serialize, Deserialize)]
#[act(visit = ["str"], expecting = "a event string <https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows>")]
pub struct Event(String);

impl Event {
    pub fn new(s: &str) -> Result<Self, Error> {
        if !EVENTS.contains(&s.to_string()) {
            Err(Error::from(ValueError::new(s)))
        } else {
            Ok(Event(s.to_string()))
        }
    }
}

impl FromStr for Event {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Event::new(s)
    }
}

#[macro_export]
macro_rules! event {
    ($e:expr) => {
        Event::new($e).unwrap()
    };
}

#[macro_export]
macro_rules! events {
    ($($e:expr),*) => {
        vec![$(Event::new($e).unwrap()),*]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let e: Event = serde_yaml::from_str("push").unwrap();
        assert_eq!(e, Event("push".to_string()));
    }

    #[test]
    fn deserialize_err() {
        let e: Result<Event, _> = serde_yaml::from_str("push1");
        assert!(e.is_err());
        assert_eq!(
            e.unwrap_err().to_string(),
            "invalid value: string \"push1\", expected a event string <https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows>"
        );
    }

    #[test]
    fn serialize() {
        let e = Event("push".to_string());
        assert_eq!(serde_yaml::to_string(&e).unwrap(), "push\n");
    }
}
