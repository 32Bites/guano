use guano_common::rowan::{NodeOrToken, GreenNode};
use guano_syntax::{parser::{
    punctuation::{left_paren, right_paren},
    Input, Res as Result,
}, SyntaxKind};

use crate::ast::prelude::*;

#[derive(Debug, Clone)]
pub struct Group {
    expr: Box<Expr>,
    span: NodeSpan,
}

pub fn group_expr<'a>(input: Input<'a>) -> Result<'a> {
    let (input, (l_paren, (l_ignored, expr, r_ignored), r_paren)) =
        tuple((left_paren, pad(expect_node(expr, "Expected expression")), expect_node(right_paren, "Expected ')'")))(input)?;

    let capacity = 3 + l_ignored.len() + r_ignored.len();
    let mut children = Vec::with_capacity(capacity);

    // (
    children.push(l_paren);
    children.extend(l_ignored);
    // `expression`
    children.push(expr);
    children.extend(r_ignored);
    // )
    children.push(r_paren);

    let mut node = NodeOrToken::Node(GreenNode::new(SyntaxKind::GROUP_EXPR.into(), children));
    node = NodeOrToken::Node(GreenNode::new(SyntaxKind::EXPR.into(), [node]));

    Ok((input, node))
}

impl Group {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Expr> {
        let (input, (span, expr)) = consumed(delimited(
            tag("("),
            padded(map(expect(Expr::parse, "Expected an expr"), |o| {
                o.unwrap_or_default()
            })),
            expect(tag(")"), "Expected a ')'"),
        ))(input)?;
        let span = span.into_node();
        let group = Self {
            expr: Box::new(expr),
            span: span.clone(),
        };

        let kind = ExprKind::Group(group);

        let expr = Expr::new(kind, span);

        Ok((input, expr))
    }
}

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a Group {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        self.expr.pretty(allocator).parens()
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.expr)
    }
}

impl Node for Group {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
