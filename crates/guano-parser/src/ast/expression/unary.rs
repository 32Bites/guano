use crate::ast::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Unary {
    operator: Spanned<UnaryOperator>,
    expr: Box<Expr>,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl Unary {
    pub fn operator(&self) -> &Spanned<UnaryOperator> {
        &self.operator
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Expr> {
        alt((
            parse_cast_expression,
            map(
                consumed(pair(UnaryOperator::parse, preceded(ignorable, Self::parse))),
                |(span, (operator, expr))| {
                    let span = span.into_node();
                    let unary = Self {
                        operator,
                        expr: Box::new(expr),
                        span: span.clone(),
                    };
                    let kind = ExprKind::Unary(unary);
                    Expr::new(kind, span)
                },
            ),
        ))(input)
    }
}

impl std::fmt::Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.operator, self.expr)
    }
}

impl Node for Unary {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
