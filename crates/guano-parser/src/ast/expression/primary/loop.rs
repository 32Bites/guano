use crate::ast::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Loop {
    block: Block,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl Loop {
    pub fn block(&self) -> &Block {
        &self.block
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Expr> {
        let (input, (span, block)) = consumed(preceded(
            Keyword::Loop,
            preceded(
                ignorable,
                map(expect(Block::parse, "Expected a block"), |o| {
                    o.unwrap_or_default()
                }),
            ),
        ))(input)?;

        let span = span.into_node();

        let loop_ = Self {
            block,
            span: span.clone(),
        };

        let kind = ExprKind::Loop(loop_);
        let expr = Expr::new(kind, span);

        Ok((input, expr))
    }
}

impl std::fmt::Display for Loop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "loop {}", self.block)
    }
}

impl Node for Loop {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
