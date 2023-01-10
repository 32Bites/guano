use crate::ast::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Group {
    expr: Box<Expr>,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
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
