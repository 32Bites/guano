use guano_common::rowan::{GreenNode, NodeOrToken};
use guano_syntax::{
    parser::{
        keyword::{kw_else, kw_if},
        Input, Res as Result,
    },
    SyntaxKind,
};

use crate::ast::prelude::*;

pub fn if_expr<'a>(input: Input<'a>) -> Result<'a> {
    let (input, (if_kw, (r_ignored, condition, l_ignored), body, else_block)) = tuple((
        kw_if(crate::ast::symbol::iden::parse_raw),
        pad(expect_node(expr, "Expected an expression")),
        expect_node(block, "Expected a block"),
        opt(pad_l(else_block)),
    ))(input)?;
    let ignored_len = r_ignored.len() + l_ignored.len();
    let other_len = [&if_kw, &condition, &body].len();
    let capacity = ignored_len + other_len;

    let mut children = Vec::with_capacity(capacity);
    children.push(if_kw);
    children.extend(r_ignored);
    children.push(condition);
    children.extend(l_ignored);
    children.push(body);

    if let Some((ignored, else_block)) = else_block {
        let reserve = 1 + ignored.len();
        children.reserve(reserve);
        children.extend(ignored);
        children.push(else_block);
    }

    let mut node = NodeOrToken::Node(GreenNode::new(SyntaxKind::IF_EXPR.into(), children));
    node = NodeOrToken::Node(GreenNode::new(SyntaxKind::EXPR.into(), [node]));

    Ok((input, node))
}

pub fn else_block<'a>(input: Input<'a>) -> Result<'a> {
    let (input, (else_kw, ignored, body)) = tuple((
        kw_else(crate::ast::symbol::iden::parse_raw),
        comments_whitespace,
        expect_node(alt((block, if_expr)), "Expected a block or if block"),
    ))(input)?;

    let capacity = 2 + ignored.len();
    let mut children = Vec::with_capacity(capacity);
    children.push(else_kw);
    children.extend(ignored);
    children.push(body);

    let node = NodeOrToken::Node(GreenNode::new(SyntaxKind::ELSE_BLOCK.into(), children));

    Ok((input, node))
}

#[derive(Debug, Clone)]
pub struct If {
    expr: Box<Expr>,
    block: Block,
    else_block: Option<Box<Expr>>,
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

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a If {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        allocator
            .text("if")
            .append(allocator.softline())
            .append(self.expr())
            .group()
            .append(allocator.softline())
            .append(self.block())
            .append(self.else_block().map(|e| allocator.softline().append(e)))
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
