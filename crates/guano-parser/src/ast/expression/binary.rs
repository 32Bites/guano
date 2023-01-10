use crate::ast::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Binary {
    operator: Spanned<BinaryOperator>,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl Binary {
    pub fn operator(&self) -> &Spanned<BinaryOperator> {
        &self.operator
    }

    pub fn lhs(&self) -> &Expr {
        &self.lhs
    }

    pub fn rhs(&self) -> &Expr {
        &self.rhs
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Expr> {
        Self::parse_assignment(input)
    }

    pub fn parse_factor(input: Span) -> Res<Expr> {
        let result = consumed(pair(
            Unary::parse,
            opt(pair(
                padded(BinaryOperator::parse_factor),
                padded(map(expect(Self::parse_factor, "Expected expr"), |o| {
                    o.unwrap_or_default()
                })),
            )),
        ))(input)?;

        Self::handle(result)
    }

    pub fn parse_term(input: Span) -> Res<Expr> {
        let result = consumed(pair(
            Self::parse_factor,
            opt(pair(
                padded(BinaryOperator::parse_term),
                padded(map(expect(Self::parse_term, "Expected expr"), |o| {
                    o.unwrap_or_default()
                })),
            )),
        ))(input)?;

        Self::handle(result)
    }

    pub fn parse_shift(input: Span) -> Res<Expr> {
        let result = consumed(pair(
            Self::parse_term,
            opt(pair(
                padded(BinaryOperator::parse_shift),
                padded(map(expect(Self::parse_shift, "Expected expr"), |o| {
                    o.unwrap_or_default()
                })),
            )),
        ))(input)?;

        Self::handle(result)
    }

    pub fn parse_bitand(input: Span) -> Res<Expr> {
        let result = consumed(pair(
            Self::parse_shift,
            opt(pair(
                padded(BinaryOperator::parse_bitand),
                padded(map(expect(Self::parse_bitand, "Expected expr"), |o| {
                    o.unwrap_or_default()
                })),
            )),
        ))(input)?;

        Self::handle(result)
    }

    pub fn parse_bitxor(input: Span) -> Res<Expr> {
        let result = consumed(pair(
            Self::parse_bitand,
            opt(pair(
                padded(BinaryOperator::parse_bitxor),
                padded(map(expect(Self::parse_bitxor, "Expected expr"), |o| {
                    o.unwrap_or_default()
                })),
            )),
        ))(input)?;

        Self::handle(result)
    }

    pub fn parse_bitor(input: Span) -> Res<Expr> {
        let result = consumed(pair(
            Self::parse_bitxor,
            opt(pair(
                padded(BinaryOperator::parse_bitor),
                padded(map(expect(Self::parse_bitor, "Expected expr"), |o| {
                    o.unwrap_or_default()
                })),
            )),
        ))(input)?;

        Self::handle(result)
    }

    pub fn parse_comparison(input: Span) -> Res<Expr> {
        let result = consumed(pair(
            Self::parse_bitor,
            opt(pair(
                padded(BinaryOperator::parse_comparison),
                padded(map(expect(Self::parse_comparison, "Expected expr"), |o| {
                    o.unwrap_or_default()
                })),
            )),
        ))(input)?;

        Self::handle(result)
    }

    pub fn parse_logand(input: Span) -> Res<Expr> {
        let result = consumed(pair(
            Self::parse_comparison,
            opt(pair(
                padded(BinaryOperator::parse_logand),
                padded(map(expect(Self::parse_logand, "Expected expr"), |o| {
                    o.unwrap_or_default()
                })),
            )),
        ))(input)?;

        Self::handle(result)
    }

    pub fn parse_logor(input: Span) -> Res<Expr> {
        let result = consumed(pair(
            Self::parse_logand,
            opt(pair(
                padded(BinaryOperator::parse_logor),
                padded(map(expect(Self::parse_logor, "Expected expr"), |o| {
                    o.unwrap_or_default()
                })),
            )),
        ))(input)?;

        Self::handle(result)
    }

    pub fn parse_assignment(input: Span) -> Res<Expr> {
        let result = consumed(pair(
            Self::parse_logor,
            opt(pair(
                padded(BinaryOperator::parse_assignment),
                padded(map(expect(Self::parse_assignment, "Expected expr"), |o| {
                    o.unwrap_or_default()
                })),
            )),
        ))(input)?;

        Self::handle(result)
    }

    fn handle(
        (input, (span, (lhs, rest))): (
            Span,
            (Span, (Expr, Option<(Spanned<BinaryOperator>, Expr)>)),
        ),
    ) -> Res<Expr> {
        match rest {
            Some((operator, rhs)) => {
                let span = span.into_node();
                let binary = Self {
                    operator,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    span: span.clone(),
                };

                let kind = ExprKind::Binary(binary);
                let expr = Expr::new(kind, span);

                Ok((input, expr))
            }
            None => Ok((input, lhs)),
        }
    }
}

impl std::fmt::Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.lhs, self.operator, self.rhs)
    }
}

impl Node for Binary {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
