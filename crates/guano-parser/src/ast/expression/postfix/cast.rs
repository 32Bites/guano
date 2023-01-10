use nom::Offset;

use crate::ast::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Cast {
    expr: Box<Expr>,
    ty: Type,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl Cast {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse<'b>(original: Span, expr: &'b Expr) -> impl FnMut(Span) -> Res<Expr> + 'b {
        let count = original.to_node().offset(&expr.span);
        let original = original.take_split(count).0;
        move |input| {
            let (input, ty) = preceded(
                padded(Keyword::As),
                map(expect(Type::parse, "Expected a type"), |t| {
                    t.unwrap_or_default()
                }),
            )(input)?;

            let start = expr.span.to_range().start;
            let end = ty.span().to_range().end;

            let byte_range = start..end;
            let byte_count = byte_range.count();

            let span = original.take(byte_count).into_node();

            let cast = Self {
                expr: Box::new(expr.clone()),
                ty,
                span: span.clone(),
            };

            let kind = ExprKind::Cast(cast);

            let expr = Expr::new(kind, span);

            Ok((input, expr))
        }
    }
}

impl std::fmt::Display for Cast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} as {}", self.expr, self.ty)
    }
}

impl Node for Cast {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
