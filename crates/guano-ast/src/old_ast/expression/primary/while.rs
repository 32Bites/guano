use guano_syntax::{parser::{keyword::kw_while, Input, Res as Result}, SyntaxKind};
use guano_common::rowan::{GreenNode, NodeOrToken};
use crate::ast::prelude::*;

pub fn while_expr<'a>(input: Input<'a>) -> Result<'a> {
    let (input, (while_kw, (l_ignored, condition, r_ignored), body)) = tuple((
        kw_while(crate::ast::symbol::iden::parse_raw),
        pad(expect_node(expr, "Expected an expression")),
        expect_node(block, "Expected a block"),
    ))(input)?;
    let capacity = 3 + l_ignored.len() + r_ignored.len();

    let mut children = Vec::with_capacity(capacity);
    children.push(while_kw);
    children.extend(l_ignored);
    children.push(condition);
    children.extend(r_ignored);
    children.push(body);

    let mut node = NodeOrToken::Node(GreenNode::new(SyntaxKind::WHILE_EXPR.into(), children));
    node = NodeOrToken::Node(GreenNode::new(SyntaxKind::EXPR.into(), [node]));

    Ok((input, node))
}

#[derive(Debug, Clone)]
pub struct While {
    expr: Box<Expr>,
    block: Block,
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

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a While {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        allocator
            .text("while")
            .append(allocator.softline())
            .append(self.expr())
            .append(allocator.softline())
            .append(self.block())
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
