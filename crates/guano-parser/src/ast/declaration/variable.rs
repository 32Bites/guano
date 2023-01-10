use crate::ast::prelude::*;

use super::modifier::Modifiers;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Var {
    modifiers: Modifiers,
    redeclarable: bool,
    iden: Iden,
    ty: Option<Type>,
    init: Option<Expr>,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl Var {
    pub fn modifiers(&self) -> &Modifiers {
        &self.modifiers
    }

    pub fn redeclarable(&self) -> bool {
        self.redeclarable
    }

    pub fn iden(&self) -> &Iden {
        &self.iden
    }

    pub fn ty(&self) -> Option<&Type> {
        self.ty.as_ref()
    }

    pub fn init(&self) -> Option<&Expr> {
        self.init.as_ref()
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(
        mut modifiers: impl Permutation<Span, Modifiers, NomError<Span>>,
    ) -> impl FnMut(Span) -> Res<Var> {
        move |input| {
            let (input, (span, (modifiers, redeclarable, iden, ty, init))) =
                consumed(Self::parse_inner(&mut modifiers))(input)?;

            let variable = Self {
                redeclarable,
                modifiers,
                iden,
                ty,
                init,
                span: span.into_node(),
            };

            Ok((input, variable))
        }
    }
    fn parse_inner<'b>(
        modifiers: &'b mut impl Permutation<Span, Modifiers, NomError<Span>>,
    ) -> impl FnMut(Span) -> Res<(Modifiers, bool, Iden, Option<Type>, Option<Expr>)> + 'b {
        move |input| {
            let (input, modifiers) = padded(|i| modifiers.permutation(i))(input)?;
            let (input, redeclarable) =
                alt((value(true, Keyword::Var), value(false, Keyword::Let)))(input)?;

            let (input, iden) = preceded(
                ignorable,
                map(expect(Iden::parse, "Missing iden"), |o| {
                    o.unwrap_or_default()
                }),
            )(input)?;

            let (input, ty) = opt(preceded(
                padded(tag(":")),
                map(expect(Type::parse, "Missing type"), |o| {
                    o.unwrap_or_default()
                }),
            ))(input)?;

            let (input, init) = opt(preceded(
                padded(tag("=")),
                map(expect(Expr::parse, "Missing initial value"), |o| {
                    o.unwrap_or_default()
                }),
            ))(input)?;

            let (input, _) = preceded(ignorable, expect(tag(";"), "Missing semicolon"))(input)?;

            Ok((input, (modifiers, redeclarable, iden, ty, init)))
        }
    }
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = if self.redeclarable { "var" } else { "let" };
        write!(f, "{keyword} {}", self.iden)?;

        if let Some(ty) = &self.ty {
            write!(f, ": {ty}")?;
        }
        if let Some(init) = &self.init {
            write!(f, " = {init}")?;
        }

        f.write_str(";")
    }
}

impl Node for Var {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
