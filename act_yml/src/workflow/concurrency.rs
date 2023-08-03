use serde::de;
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_valid::Validate;
use std::fmt;

/// Concurrency ensures that only a single job or workflow using the same concurrency group will run at a time.
#[derive(Debug, Eq, PartialEq, Validate)]
pub enum Concurrency {
    /// A string or expression concurrency group.
    #[validate(min_length = 1)]
    String(String),

    /// A concurrency group with cancel-in-progress.
    ConcurrencyWithCancel(ConcurrencyWithCancel),
}

/// A concurrency group with cancel-in-progress.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(expecting = "a map with group and cancel-in-progress")]
#[derive(Validate)]
pub struct ConcurrencyWithCancel {
    /// A string or expression concurrency group.
    #[validate(min_length = 1)]
    pub group: String,

    /// To also cancel any currently running job or workflow in the same concurrency group, specify cancel-in-progress: true.
    #[serde(rename = "cancel-in-progress", default)]
    pub cancel_in_progress: bool,
}

struct ConcurrencyVisitor;

impl<'de> Visitor<'de> for ConcurrencyVisitor {
    type Value = Concurrency;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string or a map with group and cancel-in-progress")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Concurrency::String(v.to_string()))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
            .map(Concurrency::ConcurrencyWithCancel)
    }
}

impl<'de> Deserialize<'de> for Concurrency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ConcurrencyVisitor)
    }
}

impl Serialize for Concurrency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Concurrency::String(s) => serializer.serialize_str(s),
            Concurrency::ConcurrencyWithCancel(c) => {
                if c.group.is_empty() {
                    return Err(serde::ser::Error::custom("group cannot be empty"));
                }
                c.serialize(serializer)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Concurrency;
    use serde_yaml::from_str;

    #[test]
    fn deserialize_string() {
        let s: Concurrency = from_str("staging_environment").unwrap();
        assert_eq!(s, Concurrency::String("staging_environment".to_string()));
    }

    #[test]
    fn deserialize_concurrency_with_cancel() {
        let s: Concurrency = from_str(
            r#"
        group: staging_environment
        cancel-in-progress: true
        "#,
        )
        .unwrap();
        assert_eq!(
            s,
            Concurrency::ConcurrencyWithCancel(super::ConcurrencyWithCancel {
                group: "staging_environment".to_string(),
                cancel_in_progress: true,
            })
        );
    }

    #[test]
    fn deserialize_concurrency_with_cancel_err() {
        let err = from_str::<Concurrency>(
            r#"
        cancel-in-progress: true
        "#,
        )
        .unwrap_err();
        assert_eq!(err.to_string(), "missing field `group` at line 2 column 9");
    }

    #[test]
    fn serialize_string() {
        let s = Concurrency::String("staging_environment".to_string());
        assert_eq!(serde_yaml::to_string(&s).unwrap(), "staging_environment\n");
    }

    #[test]
    fn serialize_concurrency_with_cancel() {
        let s = Concurrency::ConcurrencyWithCancel(super::ConcurrencyWithCancel {
            group: "staging_environment".to_string(),
            cancel_in_progress: true,
        });
        assert_eq!(
            serde_yaml::to_string(&s).unwrap(),
            "group: staging_environment\ncancel-in-progress: true\n"
        );
    }

    #[test]
    fn serialize_concurrency_with_cancel_err() {
        let s = Concurrency::ConcurrencyWithCancel(super::ConcurrencyWithCancel {
            group: "".to_string(),
            cancel_in_progress: true,
        });
        let err = serde_yaml::to_string(&s).unwrap_err();
        assert_eq!(err.to_string(), "group cannot be empty");
    }
}
