use crate::ast::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct While {
    expr: Box<Expr>,
    block: Block,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl While {
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
        let (input, (span, (expr, block))) = consumed(preceded(
            Keyword::While,
            pair(
                padded(map(expect(Expr::parse, "Expected an expr"), |o| {
                    o.unwrap_or_default()
                })),
                map(expect(Block::parse, "Expected a block"), |o| {
                    o.unwrap_or_default()
                }),
            ),
        ))(input)?;
        let span = span.into_node();

        let while_ = Self {
            expr: Box::new(expr),
            block,
            span: span.clone(),
        };
        let kind = ExprKind::While(while_);
        let expr = Expr::new(kind, span);

        Ok((input, expr))
    }
}

impl std::fmt::Display for While {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "while {} {}", self.expr, self.block)
    }
}

impl Node for While {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
