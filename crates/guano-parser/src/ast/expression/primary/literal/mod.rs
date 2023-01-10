use crate::ast::prelude::*;

pub mod char;
pub mod keyword;
pub mod number;
pub mod string;

use internment::ArcIntern;
pub use number::float::Float;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Lit {
    kind: LitKind,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl Lit {
    pub fn kind(&self) -> &LitKind {
        &self.kind
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Expr> {
        let (input, literal) =
            alt((keyword::parse, char::parse, string::parse, number::parse))(input)?;

        let span = literal.span.clone();
        let kind = ExprKind::Literal(literal);
        let expr = Expr::new(kind, span);

        Ok((input, expr))
    }

    fn new(kind: LitKind, span: &NodeSpan) -> Self {
        Self {
            kind,
            span: span.clone(),
        }
    }

    pub(in crate::ast) fn new_bool(boolean: bool, span: &NodeSpan) -> Self {
        Self::new(LitKind::Bool(boolean), span)
    }

    pub(in crate::ast) fn new_nil(span: &NodeSpan) -> Self {
        Self::new(LitKind::Nil, span)
    }

    pub(in crate::ast) fn new_char(character: char, span: &NodeSpan) -> Self {
        Self::new(LitKind::Char(character), span)
    }

    pub(in crate::ast) fn new_string(string: String, span: &NodeSpan) -> Self {
        Self::new(LitKind::String(ArcIntern::from(string)), span)
    }

    pub(in crate::ast) fn new_integer(integer: u128, span: &NodeSpan) -> Self {
        Self::new(LitKind::Integer(integer), span)
    }

    pub(in crate::ast) fn new_float(float: Float, span: &NodeSpan) -> Self {
        Self::new(LitKind::Float(float), span)
    }
}

impl std::fmt::Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl Node for Lit {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LitKind {
    Bool(bool),
    Integer(u128),
    Float(Float),
    String(ArcIntern<String>),
    Char(char),
    #[default]
    Nil,
}

impl std::fmt::Display for LitKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LitKind::Bool(b) => write!(f, "{b}"),
            LitKind::Integer(i) => write!(f, "{i}"),
            LitKind::Float(fl) => write!(f, "{fl}"),
            LitKind::String(s) => {
                f.write_str("\"")?;
                for c in s.chars() {
                    char::display(c, f)?;
                }

                f.write_str("\"")
            }
            LitKind::Char(c) => {
                f.write_str("'")?;
                char::display(*c, f)?;
                f.write_str("'")
            }
            LitKind::Nil => f.write_str("nil"),
        }
    }
}
