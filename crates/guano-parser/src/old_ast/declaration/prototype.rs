use crate::ast::prelude::*;

use super::{
    function::{Func, FuncBlock},
    modifier::{Modifier, Modifiers},
};

#[derive(Debug, Clone)]
pub struct Proto {
    modifiers: Modifiers,
    iden: Iden,
    extends: Vec<Path>,
    methods: Vec<Func>,
    span: NodeSpan,
}

impl Proto {
    pub fn modifiers(&self) -> &Modifiers {
        &self.modifiers
    }

    pub fn iden(&self) -> &Iden {
        &self.iden
    }

    pub fn extends(&self) -> &[Path] {
        &self.extends
    }

    pub fn methods(&self) -> &[Func] {
        &self.methods
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Self> {
        let (input, (span, (modifiers, iden, extends, methods))) =
            consumed(Self::parse_inner)(input)?;

        let proto = Self {
            modifiers,
            iden,
            extends,
            methods,
            span: span.into_node(),
        };

        Ok((input, proto))
    }

    fn parse_inner(input: Span) -> Res<(Modifiers, Iden, Vec<Path>, Vec<Func>)> {
        let (input, modifiers) = Modifiers::parse(Modifier::Pub)(input)?;
        let (input, iden) = preceded(
            padded(Keyword::Proto),
            map(expect(Iden::parse, "Expected an identifier"), |id| {
                id.unwrap_or_default()
            }),
        )(input)?;
        let (input, extends) = map(
            opt(preceded(
                padded(tag(":")),
                map(expect(Self::parse_extends, "Expected a path"), |p| {
                    p.unwrap_or_else(Vec::new)
                }),
            )),
            |e| e.unwrap_or_else(Vec::new),
        )(input)?;

        let (input, methods) = delimited(
            padded(expect(tag("{"), "Missing '{'")),
            padded(Self::parse_methods),
            expect(tag("}"), "Missing '}'"),
        )(input)?;

        Ok((input, (modifiers, iden, extends, methods)))
    }

    fn parse_extends(input: Span) -> Res<Vec<Path>> {
        map(
            pair(
                Path::parse,
                many0(preceded(
                    padded(tag("+")),
                    map(expect(Path::parse, "Expected a path"), move |p| {
                        p.unwrap_or_default()
                    }),
                )),
            ),
            |(first, rest)| {
                let mut extends = vec![first];
                extends.extend(rest.into_iter());

                extends
            },
        )(input)
    }

    fn parse_methods(input: Span) -> Res<Vec<Func>> {
        many0(padded(Func::parse(
            FuncBlock::Optional,
            (Modifier::Pub, Modifier::Static),
        )))(input)
    }
}
