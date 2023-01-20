use nom::Offset;

use crate::ast::prelude::*;

#[derive(Debug, Clone)]
pub struct Call {
    expr: Box<Expr>,
    params: Vec<Expr>,
    span: NodeSpan,
}

impl Call {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn params(&self) -> &[Expr] {
        &self.params
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse<'b>(original: Span, expr: &'b Expr) -> impl FnMut(Span) -> Res<Expr> + 'b {
        let count = original.to_node().offset(&expr.span);
        let original = original.take_split(count).0;
        move |input| {
            let (input, (span, params)) = consumed(delimited(
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
                params,
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
        let mut iter = self.params.iter().peekable();

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

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a Call {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        let mut result = self
            .expr()
            .pretty(allocator)
            .append(allocator.text("(").append(allocator.softline_()));
        let mut iter = self.params().into_iter();

        if let Some(first) = iter.next() {
            result = result.append(first);

            for doc in iter {
                result = result.append(allocator.text(",").append(allocator.softline()));
                result = result.append(doc);
            }
        }

        result = result.append(allocator.softline_().append(allocator.text(")")));

        result
    }
}
