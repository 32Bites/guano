use crate::ast::prelude::*;

use super::modifier::{Modifier, Modifiers};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Class {
    modifiers: Modifiers,
    iden: Iden,
    extends: Option<Path>,
    fields: Vec<ClassField>,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl Class {
    pub fn modifiers(&self) -> &Modifiers {
        &self.modifiers
    }

    pub fn iden(&self) -> &Iden {
        &self.iden
    }

    pub fn extends(&self) -> Option<&Path> {
        self.extends.as_ref()
    }

    pub fn fields(&self) -> &[ClassField] {
        &self.fields
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Self> {
        let (input, (span, (modifiers, iden, extends, fields))) =
            consumed(Self::parse_inner)(input)?;

        let class = Self {
            modifiers,
            iden,
            extends,
            fields,
            span: span.into_node(),
        };

        Ok((input, class))
    }

    fn parse_inner(input: Span) -> Res<(Modifiers, Iden, Option<Path>, Vec<ClassField>)> {
        let (input, modifiers) =
            terminated(Modifiers::parse(Modifier::Pub), padded(Keyword::Class))(input)?;
        let (input, iden) = map(expect(Iden::parse, "Missing identifier"), |o| {
            o.unwrap_or_default()
        })(input)?;

        let (input, extends) = padded(opt(Path::parse))(input)?;
        let (input, fields) = delimited(
            expect(tag("{"), "Expected '{'"),
            padded(Self::parse_fields),
            expect(tag("}"), "Expected '}'"),
        )(input)?;

        Ok((input, (modifiers, iden, extends, fields)))
    }

    fn parse_fields(input: Span) -> Res<Vec<ClassField>> {
        many0(padded(ClassField::parse))(input)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ClassField {
    modifiers: Modifiers,
    iden: Iden,
    ty: Type,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl ClassField {
    pub fn modifiers(&self) -> &Modifiers {
        &self.modifiers
    }

    pub fn iden(&self) -> &Iden {
        &self.iden
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Self> {
        let (input, (span, (modifiers, iden, ty))) = consumed(Self::parse_inner)(input)?;
        let field = Self {
            modifiers,
            iden,
            ty,
            span: span.into_node(),
        };

        Ok((input, field))
    }
    fn parse_inner(input: Span) -> Res<(Modifiers, Iden, Type)> {
        let (input, modifiers) = Modifiers::parse((Modifier::Pub, Modifier::Static))(input)?;
        let (input, iden) = padded(map(expect(Iden::parse, "Missing identifier"), |o| {
            o.unwrap_or_default()
        }))(input)?;
        let (input, ty) = delimited(
            expect(tag(":"), "Expected ':'"),
            padded(map(expect(Type::parse, "Expected type"), |o| {
                o.unwrap_or_default()
            })),
            expect(tag(";"), "Missing semicolon"),
        )(input)?;

        Ok((input, (modifiers, iden, ty)))
    }
}
