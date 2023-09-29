use serde::{Deserialize, Serialize};

use act_trait::{error, Error};
use error::ValueError;

// use act_derive::Deserialize;
use crate::Event;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum On {
    Event(Event),
    Events(Vec<Event>),
    // TODO: Support EventSpec
}

impl On {
    pub fn from_events(events: Vec<Event>) -> Result<Self, Error> {
        if events.is_empty() {
            Err(Error::from(ValueError::new("required at least one event")))
        } else if events.len() == 1 {
            Ok(On::Event(events.into_iter().next().unwrap()))
        } else {
            Ok(On::Events(events))
        }
    }
}

impl std::str::FromStr for On {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(On::Event(Event::from_str(s).unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{event, events};

    use super::*;

    #[test]
    fn from_events() {
        assert_eq!(
            On::from_events(events!("push")).unwrap(),
            On::Event(event!("push"))
        );
        assert_eq!(
            On::from_events(events!("push", "pull_request")).unwrap(),
            On::Events(events!("push", "pull_request")),
        );
        assert_eq!(
            On::from_events(vec![]).unwrap_err(),
            Error::from(ValueError::new("required at least one event"))
        );
    }
}
