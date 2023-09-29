use std::fmt;
use std::str::FromStr;

use serde::Serialize;

use act_derive::Deserialize;
use act_trait::ExpressionError;

use crate::InString;
use crate::InSyntax;
use crate::StringOnly;

pub static IN_BRACE_PATTERN: &str = r#"^\$\{\{(.|[\r\n])*\}\}$"#;
pub static IN_STRING_PATTERN: &str = r#"^.*\$\{\{(.|[\r\n])*\}\}.*$"#;

/// A expression string without `${{` or a expression string matching one of `^.*\$\{\{(.|[\r\n])*\}\}.*$` or `^\$\{\{(.|[\r\n])*\}\}$`
#[derive(Debug, Eq, PartialEq, Deserialize)]
#[act(visit = ["str"], expecting = "a expression string without \"${{{{\" or a expression string matching one of {IN_BRACE_PATTERN} or {IN_STRING_PATTERN}")]
pub enum Expression {
    /// A expression string without `${{`
    StringOnly(StringOnly),
    /// A string that should match `^\$\{\{(.|[\r\n])*\}\}$`
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
    pub fn string_only(s: &str) -> Result<Self, act_trait::Error> {
        StringOnly::new(s).map(Expression::StringOnly)
    }
    pub fn in_syntax(s: &str) -> Result<Self, act_trait::Error> {
        InSyntax::new(s).map(Expression::InSyntax)
    }
    pub fn in_string(s: &str) -> Result<Self, act_trait::Error> {
        InString::new(s).map(Expression::InString)
    }
}

/// A macro to create an [Expression::InString] expression
#[macro_export]
macro_rules! string_only_expr {
    ($s:expr) => {
        $crate::expression::Expression::string_only($s).unwrap()
    };
}

/// A macro to create an [Expression::InSyntax] expression
#[macro_export]
macro_rules! in_syntax_expr {
    ($s:expr) => {
        $crate::expression::Expression::in_syntax($s).unwrap()
    };
}

/// A macro to create an [Expression::InString] expression
#[macro_export]
macro_rules! in_string_expr {
    ($s:expr) => {
        $crate::expression::Expression::in_string($s).unwrap()
    };
}

impl FromStr for Expression {
    type Err = act_trait::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Expression::string_only(s)
            .ok()
            .or_else(|| Expression::in_syntax(s).ok())
            .or_else(|| Expression::in_string(s).ok())
            .ok_or_else(|| ExpressionError::expecting(s, &format!("a expression string without \"${{{{\" or a expression string matching one of {IN_BRACE_PATTERN} or {IN_STRING_PATTERN}")).into())
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
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

    use crate::{in_string_expr, in_syntax_expr, string_only_expr};

    use super::*;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_yaml::to_string(&in_syntax_expr!("${{ x }}")).unwrap(),
            "${{ x }}\n"
        );
    }

    #[test]
    fn deserialize_to_in_syntax() {
        assert_eq!(
            from_str::<Expression>(r#"${{ x }}"#).unwrap(),
            in_syntax_expr!("${{ x }}")
        );
    }

    #[test]
    fn deserialize_to_in_string() {
        assert_eq!(
            from_str::<Expression>(r#"abc${{ x }}def"#).unwrap(),
            in_string_expr!("abc${{ x }}def")
        );
    }

    #[test]
    fn deserialize_to_string_only() {
        assert_eq!(
            from_str::<Expression>("inputs.abc").unwrap(),
            string_only_expr!("inputs.abc")
        );
    }

    #[test]
    fn deserialize_err() {
        assert_eq!(
            from_str::<Expression>("abc ${{").unwrap_err().to_string(),
            format!(
                r#"invalid value: string "abc ${{{{", expected a expression string without "${{{{" or a expression string matching one of {IN_BRACE_PATTERN} or {IN_STRING_PATTERN}"#
            )
        );
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", in_syntax_expr!("${{ x }}")), r#"${{ x }}"#);
    }
}
