use crate::ast::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct List {
    expressions: Vec<Expr>,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl List {
    pub fn expressions(&self) -> &[Expr] {
        &self.expressions
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Expr> {
        let (input, (span, expressions)) = consumed(delimited(
            tag("["),
            padded(map(
                expect(
                    separated_list0(tag(","), padded(Expr::parse)),
                    "Invalid list",
                ),
                |o| o.unwrap_or_default(),
            )),
            expect(tag("]"), "Expected a ']'"),
        ))(input)?;

        let span = span.into_node();

        let list = Self {
            expressions,
            span: span.clone(),
        };

        let kind = ExprKind::List(list);
        let expr = Expr::new(kind, span);

        Ok((input, expr))
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        let mut iter = self.expressions.iter().peekable();

        while let Some(expr) = iter.next() {
            expr.fmt(f)?;
            if iter.peek().is_some() {
                f.write_str(", ")?;
            }
        }

        f.write_str("]")
    }
}

impl Node for List {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
