use nom::Offset;

use crate::ast::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Field {
    expr: Box<Expr>,
    iden: Iden,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl Field {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn iden(&self) -> &Iden {
        &self.iden
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse<'b>(original: Span, expr: &'b Expr) -> impl FnMut(Span) -> Res<Expr> + 'b {
        let count = original.to_node().offset(&expr.span);
        let original = original.take_split(count).0;
        move |input| {
            let (input, iden) = preceded(
                padded(tag(".")),
                map(expect(Iden::parse, "Expected an iden"), |o| {
                    o.unwrap_or_default()
                }),
            )(input)?;

            // Determine the byte range of the field access operation.
            let start = expr.span.to_range().start;
            let end = iden.span().to_range().end;
            let byte_range = start..end;

            // Determine the byte count.
            let byte_count = byte_range.count();

            // Using the byte count and the modified original span, determine the span of the full expr.
            let span = original.take(byte_count).into_node();

            let field = Self {
                expr: Box::new(expr.clone()),
                iden: iden,
                span: span.clone(),
            };
            let kind = ExprKind::Field(field);

            let expr = Expr::new(kind, span);

            Ok((input, expr))
        }
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.expr, self.iden)
    }
}

impl Node for Field {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
