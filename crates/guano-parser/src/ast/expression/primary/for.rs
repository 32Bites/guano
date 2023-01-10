use crate::ast::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct For {
    iden: Iden,
    expr: Box<Expr>,
    block: Block,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl For {
    pub fn iden(&self) -> &Iden {
        &self.iden
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn block(&self) -> &Block {
        &self.block
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Expr> {
        let (input, (span, (iden, expr, block))) = consumed(preceded(
            Keyword::For,
            tuple((
                padded(map(expect(Iden::parse, "Expected an iden"), |o| {
                    o.unwrap_or_default()
                })),
                preceded(
                    expect(Keyword::In, "Expected 'in'"),
                    padded(map(expect(Expr::parse, "Expected an expr"), |o| {
                        o.unwrap_or_default()
                    })),
                ),
                map(expect(Block::parse, "Expected a block"), |o| {
                    o.unwrap_or_default()
                }),
            )),
        ))(input)?;

        let span = span.into_node();
        let for_ = Self {
            iden: iden,
            expr: Box::new(expr),
            block,
            span: span.clone(),
        };
        let kind = ExprKind::For(for_);
        let expr = Expr::new(kind, span);

        Ok((input, expr))
    }
}

impl std::fmt::Display for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "for {} in {} {}", self.iden, self.expr, self.block)
    }
}

impl Node for For {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
