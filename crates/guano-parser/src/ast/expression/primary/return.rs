use crate::ast::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Return {
    expr: Option<Box<Expr>>,
    #[cfg_attr(feature = "serde", serde(skip))]
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
