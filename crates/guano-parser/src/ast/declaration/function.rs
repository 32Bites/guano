use crate::ast::prelude::*;

use super::modifier::Modifiers;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Func {
    modifiers: Modifiers,
    iden: Iden,
    params: Vec<FuncParam>,
    ty: Option<Type>,
    block: Option<Block>,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl Func {
    pub fn modifiers(&self) -> &Modifiers {
        &self.modifiers
    }

    pub fn iden(&self) -> &Iden {
        &self.iden
    }

    pub fn params(&self) -> &[FuncParam] {
        &self.params
    }

    pub fn ty(&self) -> Option<&Type> {
        self.ty.as_ref()
    }

    pub fn block(&self) -> Option<&Block> {
        self.block.as_ref()
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(
        block_kind: FuncBlock,
        mut modifiers: impl Permutation<Span, Modifiers, NomError<Span>>,
    ) -> impl FnMut(Span) -> Res<Func> {
        move |input| {
            let (input, (span, (modifiers, iden, params, ty, block))) =
                consumed(Self::parse_inner(block_kind, &mut modifiers))(input)?;

            let function = Self {
                modifiers,
                iden,
                params,
                ty,
                block,
                span: span.into_node(),
            };

            Ok((input, function))
        }
    }

    fn parse_inner<'b>(
        mut block_kind: FuncBlock,
        modifiers: &'b mut impl Permutation<Span, Modifiers, NomError<Span>>,
    ) -> impl FnMut(Span) -> Res<(Modifiers, Iden, Vec<FuncParam>, Option<Type>, Option<Block>)> + 'b
    {
        move |input| {
            let (input, modifiers) = padded(|i| modifiers.permutation(i))(input)?;
            let (input, iden) = preceded(
                preceded(ignorable, Keyword::Fun),
                padded(map(expect(Iden::parse, "Expected an identifier"), |o| {
                    o.unwrap_or_default()
                })),
            )(input)?;
            let (input, params) = Self::parse_parameters(input)?;
            let (input, ty) = opt(Self::parse_return_type)(input)?;
            let (input, block) = block_kind.parse(input)?;

            Ok((input, (modifiers, iden, params, ty, block)))
        }
    }

    fn parse_parameters(input: Span) -> Res<Vec<FuncParam>> {
        let paren = preceded(
            ignorable,
            delimited(
                tag("("),
                alt((
                    map(
                        pair(
                            padded(FuncParam::parse),
                            many0(preceded(
                                padded(tag(",")),
                                padded(map(
                                    expect(FuncParam::parse, "Invalid function parameter"),
                                    move |o| o.unwrap_or_default(),
                                )),
                            )),
                        ),
                        |(first, rest)| {
                            let mut params = vec![first];
                            params.extend(rest);
                            params
                        },
                    ),
                    map(ignorable, |_| vec![]),
                )),
                expect(tag(")"), "Expected a ')'"),
            ),
        );
        alt((paren, success(vec![])))(input)
    }

    fn parse_return_type(input: Span) -> Res<Type> {
        let (input, ty) = preceded(
            pair(ignorable, tag("->")),
            map(expect(Type::parse, "Expected a return type"), move |o| {
                o.unwrap_or_default()
            }),
        )(input)?;

        Ok((input, ty))
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FuncParam {
    iden: Iden,
    ty: Type,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl FuncParam {
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
        map(
            consumed(separated_pair(Iden::parse, padded(tag(":")), Type::parse)),
            |(span, (iden, ty))| Self {
                iden,
                ty,
                span: span.into_node(),
            },
        )(input)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FuncBlock {
    /// A block is required, always
    Needed,
    /// A block may or may not exist.
    Optional,
    /// A block must not exist.
    None,
}

impl FuncBlock {
    fn parse_needed(input: Span) -> Res<Option<Block>> {
        map(preceded(ignorable, Block::parse), |b| Some(b))(input)
    }

    fn parse_optional(input: Span) -> Res<Option<Block>> {
        alt((Self::parse_needed, Self::parse_optional))(input)
    }

    fn parse_none(input: Span) -> Res<Option<Block>> {
        value(None, preceded(ignorable, tag(";")))(input)
    }
}

impl Parser<Span, Option<Block>, NomError<Span>> for FuncBlock {
    fn parse(&mut self, input: Span) -> nom::IResult<Span, Option<Block>, NomError<Span>> {
        match self {
            FuncBlock::Needed => map(expect(Self::parse_needed, "Expected a block"), |o| {
                o.flatten()
            })(input),
            FuncBlock::Optional => map(expect(Self::parse_optional, "Expected a semicolon"), |o| {
                o.flatten()
            })(input),
            FuncBlock::None => map(
                expect(Self::parse_none, "Expected a block or semicolon"),
                |o| o.flatten(),
            )(input),
        }
    }
}
