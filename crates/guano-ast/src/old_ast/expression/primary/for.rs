use crate::ast::prelude::*;
use guano_common::rowan::{GreenNode, NodeOrToken};
use guano_syntax::{
    error,
    parser::{
        keyword::{kw_for, kw_in},
        Input, Res as Result,
    },
    SyntaxKind,
};

pub fn for_expr<'a>(input: Input<'a>) -> Result<'a> {
    let r#in = pad(tuple((
        expect_node(crate::ast::symbol::iden::parse, "Expected an identifier"),
        pad(expect_node(
            kw_in(crate::ast::symbol::iden::parse_raw),
            "Expected 'in'",
        )),
        expect_node(expr, "Expected an expression"),
    )));

    let (input, (for_kw, (lo_ignored, (iden, (li_ignored, in_kw, ri_ignored), expr), ro_ignored), body)) = tuple((
        kw_for(crate::ast::symbol::iden::parse_raw),
        r#in,
        expect_node(block, "Expected a block"),
    ))(input)?;

    let ignored_len = lo_ignored.len() + ro_ignored.len() + li_ignored.len() + ri_ignored.len();
    let other_len = [&for_kw, &iden, &in_kw, &expr, &body].len();
    let capacity = other_len + ignored_len;

    let mut children = Vec::with_capacity(capacity);
    // for
    children.push(for_kw);
    children.extend(lo_ignored);
    // `identifier`
    children.push(iden);
    children.extend(li_ignored);
    // in
    children.push(in_kw);
    children.extend(ri_ignored);
    // `expression`
    children.push(expr);
    children.extend(ro_ignored);
    // `block`
    children.push(body);

    let mut node = NodeOrToken::Node(GreenNode::new(SyntaxKind::FOR_EXPR.into(), children));
    node = NodeOrToken::Node(GreenNode::new(SyntaxKind::EXPR.into(), [node]));

    Ok((input, node))
}

#[derive(Debug, Clone)]
pub struct For {
    iden: Iden,
    expr: Box<Expr>,
    block: Block,
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

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a For {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        allocator
            .text("for")
            .append(allocator.softline())
            .append(self.iden())
            .append(allocator.softline())
            .append("in")
            .append(allocator.softline())
            .append(self.expr())
            .group()
            .append(allocator.space())
            .append(self.block())
    }
}

impl std::fmt::Display for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "for {{}} in {} {}", /* , self.iden */
            self.expr, self.block
        )
    }
}

impl Node for For {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
