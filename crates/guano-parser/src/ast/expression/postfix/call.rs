use nom::Offset;

use crate::ast::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Call {
    expr: Box<Expr>,
    parameters: Vec<Expr>,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl Call {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn parameters(&self) -> &[Expr] {
        &self.parameters
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse<'b>(original: Span, expr: &'b Expr) -> impl FnMut(Span) -> Res<Expr> + 'b {
        let count = original.to_node().offset(&expr.span);
        let original = original.take_split(count).0;
        move |input| {
            let (input, (span, parameters)) = consumed(delimited(
                preceded(ignorable, tag("(")),
                map(
                    expect(
                        separated_list0(tag(","), padded(Expr::parse)),
                        "Invalid parameters",
                    ),
                    |o| o.unwrap_or_default(),
                ),
                expect(tag(")"), "Expected a ')'"),
            ))(input)?;

            let start = expr.span.to_range().start;
            let end = span.to_range().end;

            let byte_range = start..end;
            let byte_count = byte_range.count();

            let span = original.take(byte_count).into_node();

            let call = Self {
                expr: Box::new(expr.clone()),
                parameters,
                span: span.clone(),
            };
            let kind = ExprKind::Call(call);
            let expr = Expr::new(kind, span);

            Ok((input, expr))
        }
    }
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.expr.fmt(f)?;
        f.write_str("(")?;
        let mut iter = self.parameters.iter().peekable();

        while let Some(parameter) = iter.next() {
            parameter.fmt(f)?;
            if iter.peek().is_some() {
                f.write_str(", ")?;
            }
        }

        f.write_str(")")
    }
}

impl Node for Call {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
