use crate::ast::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct If {
    expr: Box<Expr>,
    block: Block,
    else_block: Option<Box<Expr>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl If {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn block(&self) -> &Block {
        &self.block
    }

    pub fn else_block(&self) -> Option<&Expr> {
        self.else_block.as_deref()
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Expr> {
        let else_block = preceded(
            padded(Keyword::Else),
            map(
                expect(
                    alt((parse_block_expression, Self::parse)),
                    "Expected another if block or a block",
                ),
                |o| o.unwrap_or_default(),
            ),
        );
        let (input, (span, (expr, block, else_block))) = consumed(preceded(
            Keyword::If,
            tuple((
                padded(map(expect(Expr::parse, "Expected an expr"), |o| {
                    o.unwrap_or_default()
                })),
                map(expect(Block::parse, "Expected a block"), |o| {
                    o.unwrap_or_default()
                }),
                opt(else_block),
            )),
        ))(input)?;

        let span = span.into_node();

        let if_ = Self {
            expr: Box::new(expr),
            block,
            else_block: else_block.map(Box::new),
            span: span.clone(),
        };

        let kind = ExprKind::If(if_);
        let expr = Expr::new(kind, span);

        Ok((input, expr))
    }
}

impl std::fmt::Display for If {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {} {}", self.expr, self.block)?;
        if let Some(else_block) = &self.else_block {
            write!(f, " {}", else_block)?;
        }

        Ok(())
    }
}

impl Node for If {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
