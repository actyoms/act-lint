#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged, expecting = "expected one of string, number or boolean")]
pub enum StrNumBool {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

#[cfg(test)]
mod tests {
    use serde_yaml::from_str;

    use super::*;

    #[test]
    fn deserialize_int_ok() {
        let e: StrNumBool = from_str("123").unwrap();
        assert_eq!(e, StrNumBool::Int(123));
    }

    #[test]
    fn deserialize_float_ok() {
        let e: StrNumBool = from_str("123.456").unwrap();
        assert_eq!(e, StrNumBool::Float(123.456));
    }

    #[test]
    fn deserialize_bool_ok() {
        let e: StrNumBool = from_str("true").unwrap();
        assert_eq!(e, StrNumBool::Bool(true));
    }

    #[test]
    fn deserialize_string_ok() {
        let e: StrNumBool = from_str("abc").unwrap();
        assert_eq!(e, StrNumBool::String("abc".to_string()));
    }

    #[test]
    fn deserialize_err() {
        let err = from_str::<StrNumBool>("null").unwrap_err();
        assert_eq!(err.to_string(), "expected one of string, number or boolean");
    }
}
