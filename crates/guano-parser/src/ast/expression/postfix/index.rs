use nom::Offset;

use crate::ast::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Index {
    expr: Box<Expr>,
    index: Box<Expr>,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.expr, self.index)
    }
}

impl Index {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn index(&self) -> &Expr {
        &self.index
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse<'b>(original: Span, expr: &'b Expr) -> impl FnMut(Span) -> Res<Expr> + 'b {
        let count = original.to_node().offset(&expr.span);
        let original = original.take_split(count).0;
        move |input| {
            let (input, (span, index)) = consumed(delimited(
                preceded(ignorable, tag("[")),
                padded(map(expect(Expr::parse, "Expected an expr"), |o| {
                    o.unwrap_or_default()
                })),
                expect(tag("]"), "Expected a ']'"),
            ))(input)?;
            let start = expr.span.to_range().start;
            let end = span.to_range().end;

            let byte_range = start..end;
            let byte_count = byte_range.count();

            let span = original.take(byte_count).into_node();

            let index = Self {
                expr: Box::new(expr.clone()),
                index: Box::new(index),
                span: span.clone(),
            };

            let kind = ExprKind::Index(index);
            let expr = Expr::new(kind, span);

            Ok((input, expr))
        }
    }
}

impl Node for Index {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
