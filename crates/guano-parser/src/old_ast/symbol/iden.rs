use std::{
    fmt,
    io::{self, Write},
    ops::Deref,
};

use guano_syntax::{
    leaf,
    parser::{Input, Res as Ress},
    SyntaxKind,
};
use internment::ArcIntern;
use nom::IResult;
use pretty::Pretty;
use serde::{de::Visitor, Serialize, Serializer};

use crate::ast::prelude::*;

#[derive(Debug, Clone)]
pub struct Iden {
    data: ArcIntern<String>,
    span: NodeSpan,
}

impl Default for Iden {
    fn default() -> Self {
        Self {
            data: ArcIntern::from_ref(""),
            span: NodeSpan::default(),
        }
    }
}

pub fn parse_raw<'a>(input: Input<'a>) -> Ress<'a, Input<'a>> {
    recognize(pair(
        alt((alpha1::<Input, _>, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

pub fn parse<'a>(input: Input<'a>) -> Ress<'a> {
    map(
        verify(parse_raw, |i| {
            SyntaxKind::from_keyword(i.fragment()).is_none()
        }),
        |i| leaf(SyntaxKind::IDEN, i.fragment()),
    )(input)
}

impl Iden {
    pub fn as_str(&self) -> &str {
        self.data.as_ref()
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse_raw(input: Span) -> Res<Iden> {
        let (input, span) = map(
            recognize(pair(
                alt((alpha1::<Span, _>, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            )),
            |s| s.to_node(),
        )(input)?;
        Ok((
            input,
            Self {
                data: ArcIntern::from_ref(span.as_str()),
                span,
            },
        ))
    }

    pub fn new_parse_raw(input: &str) -> Res<Iden> {
        /*         let (input, span) = map(
            recognize(pair(
                alt((alpha1::<Span, _>, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            )),
            |s| s.to_node(),
        )(input)?;
        Ok((
            input,
            Self {
                data: ArcIntern::from_ref(span.as_str()),
                span,
            },
        )) */

        todo!()
    }

    pub fn parse(input: Span) -> Res<Iden> {
        let mut iden = map(
            map_opt(Self::parse_raw, |s| match Keyword::from_str(s.as_ref()) {
                Some(k) => match k.is_primitive() {
                    true => Some(s),
                    false => None,
                },
                None => Some(s),
            }),
            |s| s,
        );

        iden(input)
    }
}

impl PartialEq for Iden {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<&str> for Iden {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<Keyword> for Iden {
    fn eq(&self, other: &Keyword) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Iden {}

impl std::hash::Hash for Iden {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl AsRef<str> for Iden {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for Iden {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl From<Iden> for String {
    fn from(iden: Iden) -> Self {
        iden.as_str().to_owned()
    }
}

/* impl std::fmt::Display for Iden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let allocator = pretty::Arena::<()>::new();
        let doc = self.pretty(&allocator).into_doc();
        doc.render_fmt(usize::MAX, f)?;


        Ok(())
    }
} */

/* impl Node for Iden {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
} */

impl Serialize for Iden {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Iden {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(IdenVisitor)
    }
}

struct IdenVisitor;

impl<'de> Visitor<'de> for IdenVisitor {
    type Value = Iden;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Expected an identifier")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // TODO: Ensure that the string is indeed an identifier
        Ok(Iden {
            data: ArcIntern::from_ref(v),
            span: NodeSpan::default(),
        })
    }
}

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a Iden {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        allocator.text(self.as_str())
    }
}
