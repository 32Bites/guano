use crate::ast::prelude::*;
use guano_common::rowan::{GreenNode, NodeOrToken};
use guano_syntax::{
    parser::{keyword::kw_return, Input, Res as Result},
    SyntaxKind,
};

pub fn return_expr<'a>(input: Input<'a>) -> Result<'a> {
    let (input, (return_kw, value)) = pair(
        kw_return(crate::ast::symbol::iden::parse_raw),
        opt(pad_l(expr)),
    )(input)?;
    
    let mut children = vec![return_kw];

    if let Some((ignored, value)) = value {
        let reserve = 1 + ignored.len();
        children.reserve(reserve);
        children.extend(ignored);
        children.push(value);
    }

    let mut node = NodeOrToken::Node(GreenNode::new(SyntaxKind::RETURN_EXPR.into(), children));
    node = NodeOrToken::Node(GreenNode::new(SyntaxKind::EXPR.into(), [node]));

    Ok((input, node))
}

#[derive(Debug, Clone)]
pub struct Return {
    expr: Option<Box<Expr>>,
    span: NodeSpan,
}

impl Return {
    pub fn expr(&self) -> Option<&Expr> {
        self.expr.as_deref()
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Expr> {
        let (input, (span, expr)) =
            consumed(preceded(pair(Keyword::Return, ignorable), opt(Expr::parse)))(input)?;

        let span = span.into_node();

        let ret = Self {
            expr: expr.map(Box::new),
            span: span.clone(),
        };

        let kind = ExprKind::Return(ret);
        let expr = Expr::new(kind, span);

        Ok((input, expr))
    }
}

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a Return {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        allocator
            .text("return")
            .append(self.expr().map(|e| allocator.softline().append(e)))
            .group()
    }
}

impl std::fmt::Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.expr {
            Some(expr) => write!(f, "return {expr}"),
            None => f.write_str("return"),
        }
    }
}

impl Node for Return {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
