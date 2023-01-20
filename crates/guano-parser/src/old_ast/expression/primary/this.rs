use guano_common::rowan::{NodeOrToken, GreenNode};
use guano_syntax::{parser::{Input, Res as Result, punctuation::at}, SyntaxKind};

use crate::ast::prelude::*;


pub fn this_expr<'a>(input: Input<'a>) -> Result<'a> {
    let (input, (at, iden)) = pair(at, expect_node(crate::ast::symbol::iden::parse, "Expected identifier"))(input)?;
    let mut node = NodeOrToken::Node(GreenNode::new(SyntaxKind::THIS_EXPR.into(), [at, iden]));
    node = NodeOrToken::Node(GreenNode::new(SyntaxKind::EXPR.into(), [node]));

    Ok((input, node))
}

#[derive(Debug, Clone)]
pub struct This {
    iden: Iden,
    span: NodeSpan,
}

impl This {
    pub fn iden(&self) -> &Iden {
        &self.iden
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Expr> {
        let (input, (span, iden)) = consumed(preceded(
            tag("@"),
            map(expect(Iden::parse, "Expected a iden"), |iden| {
                iden.unwrap_or_default()
            }),
        ))(input)?;

        let span = span.into_node();

        let this = This {
            iden,
            span: span.clone(),
        };
        let kind = ExprKind::This(this);
        let expr = Expr::new(kind, span);

        Ok((input, expr))
    }
}

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a This {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        allocator.text("@").append(self.iden())
    }
}

impl std::fmt::Display for This {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{{}}", /* self.iden */)
    }
}

impl Node for This {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
