use guano_common::rowan::{GreenNode, NodeOrToken};
use guano_syntax::{
    parser::{keyword::kw_loop, Input, Res as Result},
    SyntaxKind,
};

use crate::ast::prelude::*;

#[derive(Debug, Clone)]
pub struct Loop {
    block: Block,
    span: NodeSpan,
}

pub fn loop_expr<'a>(input: Input<'a>) -> Result<'a> {
    let (input, (kw, ignored, body)) = tuple((
        kw_loop(crate::ast::symbol::iden::parse_raw),
        comments_whitespace,
        block,
    ))(input)?;
    let capacity = 2 + ignored.len();

    let mut children = Vec::with_capacity(capacity);
    children.push(kw);
    children.extend(ignored);
    children.push(body);

    let mut node = NodeOrToken::Node(GreenNode::new(SyntaxKind::LOOP_EXPR.into(), children));
    node = NodeOrToken::Node(GreenNode::new(SyntaxKind::EXPR.into(), [node]));

    Ok((input, node))
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

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a Loop {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        allocator
            .text("loop")
            .append(allocator.softline())
            .append(self.block())
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
