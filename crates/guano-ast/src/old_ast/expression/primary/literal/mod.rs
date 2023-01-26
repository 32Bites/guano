use std::sync::Arc;

use crate::ast::prelude::*;

pub mod char;
pub mod keyword;
pub mod number;
pub mod string;

use guano_interner::{InternedInteger, InternedString};
use guano_common::once_cell::sync::Lazy;
use guano_syntax::{
    parser::{wrap, Input, Res as Result},
    SyntaxKind,
};
use internment::ArcIntern;
pub use number::float::Float;
use rug::float::Special;

use self::{
    char::{character_literal, CharExt},
    number::{float::FloatExt, integer::IntegerExt, number_literal},
    string::{string_literal, StringExt},
};

static INF: Lazy<Arc<rug::Float>> =
    Lazy::new(|| Arc::new(rug::Float::with_val(52, Special::Infinity)));
static NAN: Lazy<Arc<rug::Float>> = Lazy::new(|| Arc::new(rug::Float::with_val(52, Special::Nan)));

pub trait LiteralExt {
    fn value(&self) -> Option<LiteralValue>;
}

impl LiteralExt for guano_syntax::nodes::Literal {
    fn value(&self) -> Option<LiteralValue> {
        let value = if self.is_nil() {
            LiteralValue::Nil
        } else if self.is_inf() {
            LiteralValue::Float(INF.clone())
        } else if self.is_nan() {
            LiteralValue::Float(NAN.clone())
        } else if let Some(boolean) = self.boolean() {
            LiteralValue::Boolean(boolean)
        } else if let Some(integer) = self.integer() {
            LiteralValue::Integer(integer.value()?)
        } else if let Some(float) = self.float() {
            LiteralValue::Float(float.value()?)
        } else if let Some(string) = self.string() {
            LiteralValue::String(string.value()?)
        } else if let Some(character) = self.char() {
            LiteralValue::Character(character.value()?)
        } else {
            return None;
        };

        Some(value)
    }
}

pub fn literal<'a>(input: Input<'a>) -> Result<'a> {
    wrap(
        alt((
            number_literal,
            keyword::keyword,
            character_literal,
            string_literal,
        )),
        SyntaxKind::LITERAL,
    )(input)
}

pub fn literal_expr<'a>(input: Input<'a>) -> Result<'a> {
    wrap(literal, SyntaxKind::EXPR)(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Integer(InternedInteger),
    Float(Arc<rug::Float>),
    String(InternedString),
    Boolean(bool),
    Character(char),
    Nil,
}

#[derive(Debug, Clone, Default)]
pub struct Lit {
    kind: LitKind,
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

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a Lit {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        self.kind.pretty(allocator)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LitKind {
    Bool(bool),
    Integer(u128),
    Float(Float),
    String(ArcIntern<String>),
    Char(char),
    #[default]
    Nil,
}

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a LitKind {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        match self {
            LitKind::Bool(b) => allocator.as_string(b),
            LitKind::Integer(i) => allocator.as_string(i),
            LitKind::Float(f) => allocator.as_string(f),
            LitKind::String(s) => {
                let mut buf = String::new();

                for c in s.chars() {
                    char::display(c, &mut buf).expect("This should never fail");
                }

                allocator.text(buf).double_quotes()
            }
            LitKind::Char(c) => {
                let mut buf = String::new();
                char::display(*c, &mut buf).expect("This should never fail");

                allocator.text(buf).single_quotes()
            }
            LitKind::Nil => allocator.text("nil"),
        }
    }
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
