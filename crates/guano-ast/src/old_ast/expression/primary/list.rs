use guano_common::rowan::{GreenNode, NodeOrToken};
use guano_syntax::{
    parser::{punctuation::comma, Input, Res as Result},
    SyntaxKind,
};

use crate::ast::prelude::*;

pub fn list_expr<'a>(input: Input<'a>) -> Result<'a> {
    todo!()
}

pub fn list_expr_item<'a>(input: Input<'a>) -> Result<'a> {
    let (input, (comma, ignored, value)) = tuple((
        comma,
        comments_whitespace,
        expect_node(expr, "Expected expression"),
    ))(input)?;

    let capacity = 2 + ignored.len();
    let mut children = Vec::with_capacity(capacity);
    children.push(comma);
    children.extend(ignored);
    children.push(value);

    let node = NodeOrToken::Node(GreenNode::new(SyntaxKind::LIST_EXPR_ITEM.into(), children));

    Ok((input, node))
}

#[derive(Debug, Clone)]
pub struct List {
    values: Vec<Expr>,
    span: NodeSpan,
}

impl List {
    pub fn exprs(&self) -> &[Expr] {
        &self.values
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Expr> {
        let (input, (span, values)) = consumed(delimited(
            tag("["),
            padded(map(
                expect(
                    separated_list0(tag(","), padded(Expr::parse)),
                    "Invalid list",
                ),
                |o| o.unwrap_or_default(),
            )),
            expect(tag("]"), "Expected a ']'"),
        ))(input)?;

        let span = span.into_node();

        let list = Self {
            values,
            span: span.clone(),
        };

        let kind = ExprKind::List(list);
        let expr = Expr::new(kind, span);

        Ok((input, expr))
    }
}

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a List {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        let mut result = allocator.text("[").append(allocator.softline_());
        let mut iter = self.exprs().into_iter();

        if let Some(first) = iter.next() {
            result = result.append(first);

            for doc in iter {
                result = result.append(allocator.text(",").append(allocator.softline()));
                result = result.append(doc);
            }
        }

        result = result.append(allocator.softline_().append(allocator.text("]")));

        result
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        let mut iter = self.values.iter().peekable();

        while let Some(expr) = iter.next() {
            expr.fmt(f)?;
            if iter.peek().is_some() {
                f.write_str(", ")?;
            }
        }

        f.write_str("]")
    }
}

impl Node for List {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
