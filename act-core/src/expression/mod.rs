use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{EnumAccess, Error, Visitor};

pub use in_string::InString;
pub use in_syntax::InSyntax;
pub use string_only::StringOnly;

pub use crate::{in_string, in_syntax, string_only};

pub mod in_string;
pub mod in_syntax;
pub mod string_only;

pub static IN_BRACE_PATTERN: &str = r#"^\$\{\{(.|[\r\n])*\}\}$"#;
pub static IN_STRING_PATTERN: &str = r#"^.*\$\{\{(.|[\r\n])*\}\}.*$"#;

/// A expression string without `${{` or a expression string matching one of `^.*\$\{\{(.|[\r\n])*\}\}.*$` or `^\$\{\{(.|[\r\n])*\}\}$`
#[derive(Debug, Eq, PartialEq)]
pub enum Expression {
    /// A expression string without `${{`
    StringOnly(StringOnly),
    /// A string that should match `^.*\$\{\{(.|[\r\n])*\}\}.*$`
    InSyntax(InSyntax),
    /// A string that should match `^.*\$\{\{(.|[\r\n])*\}\}.*$`
    InString(InString),
}

impl Expression {
    pub fn as_str(&self) -> &str {
        match self {
            Expression::StringOnly(s) => s.as_str(),
            Expression::InSyntax(s) => s.as_str(),
            Expression::InString(s) => s.as_str(),
        }
    }
    pub fn string_only(s: &str) -> Result<Self, crate::Error> {
        string_only!(s).map(Expression::StringOnly)
    }
    pub fn in_syntax(s: &str) -> Result<Self, crate::Error> {
        in_syntax!(s).map(Expression::InSyntax)
    }
    pub fn in_string(s: &str) -> Result<Self, crate::Error> {
        in_string!(s).map(Expression::InString)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

struct ExpressionVisitor;

impl<'de> Visitor<'de> for ExpressionVisitor {
    type Value = Expression;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "a expression string without \"${{{{\" or a expression string matching one of {IN_BRACE_PATTERN} or {IN_STRING_PATTERN}",
        ))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let x = Expression::string_only(v)
                .ok()
                .or_else(|| Expression::in_syntax(v).ok())
                .or_else(|| Expression::in_string(v).ok());
        if let Some(x) = x {
            Ok(x)
        } else {
            Err(Error::invalid_value(serde::de::Unexpected::Str(v), &self))
        }
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        data.variant().map(|(variant, _)| self.visit_str(variant))?
    }
}

impl<'de> Deserialize<'de> for Expression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_enum(
            "Expression",
            &["StringOnly", "InSyntax", "InString"],
            ExpressionVisitor,
        )
    }
}

impl Serialize for Expression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Expression::StringOnly(s) => s.serialize(serializer),
            Expression::InSyntax(s) => s.serialize(serializer),
            Expression::InString(s) => s.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml::from_str;

    use super::*;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_yaml::to_string(&Expression::in_syntax("${{ x }}").unwrap()).unwrap(),
            "${{ x }}\n"
        );
    }

    #[test]
    fn deserialize_to_in_syntax() {
        assert_eq!(
            from_str::<Expression>(r#"${{ x }}"#).unwrap(),
            Expression::in_syntax("${{ x }}").unwrap()
        );
    }

    #[test]
    fn deserialize_to_in_string() {
        assert_eq!(
            from_str::<Expression>(r#"abc${{ x }}def"#).unwrap(),
            Expression::in_string("abc${{ x }}def").unwrap()
        );
    }

    #[test]
    fn deserialize_to_string_only() {
        assert_eq!(
            from_str::<Expression>("inputs.abc").unwrap(),
            Expression::string_only("inputs.abc").unwrap()
        );
    }

    #[test]
    fn deserialize_err() {
        assert_eq!(
            from_str::<Expression>("abc ${{").unwrap_err().to_string(),
            format!(r#"invalid value: string "abc ${{{{", expected a expression string without "${{{{" or a expression string matching one of {IN_BRACE_PATTERN} or {IN_STRING_PATTERN}"#)
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", Expression::in_syntax("${{ x }}").unwrap()),
            r#"${{ x }}"#
        );
    }
}
