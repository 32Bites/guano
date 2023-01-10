use crate::ast::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct This {
    iden: Iden,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl This {
    pub fn iden(&self) -> &Iden {
        &self.iden
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Expr> {
        let (input, (span, iden)) = consumed(preceded(
            tag("@"),
            map(expect(Iden::parse, "Expected a iden"), |iden| {
                iden.unwrap_or_default()
            }),
        ))(input)?;

        let span = span.into_node();

        let this = This {
            iden,
            span: span.clone(),
        };
        let kind = ExprKind::This(this);
        let expr = Expr::new(kind, span);

        Ok((input, expr))
    }
}

impl std::fmt::Display for This {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.iden)
    }
}

impl Node for This {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
