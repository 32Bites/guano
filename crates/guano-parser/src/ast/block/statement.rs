use crate::ast::{declaration::variable::Var, prelude::*};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Statement {
    kind: StatementKind,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl Statement {
    pub fn kind(&self) -> &StatementKind {
        &self.kind
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Self> {
        alt((
            Self::parse_empty,
            Self::parse_expression,
            Self::parse_variable,
        ))(input)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.kind, StatementKind::Empty)
    }

    pub fn parse_empty(input: Span) -> Res<Statement> {
        let (input, span) = recognize(many1_count(preceded(ignorable, tag(";"))))(input)?;

        let statement = Self {
            kind: StatementKind::Empty,
            span: span.into_node(),
        };

        Ok((input, statement))
    }

    pub fn parse_variable(input: Span) -> Res<Statement> {
        let (input, (span, var)) = consumed(Var::parse(()))(input)?;

        let statement = Self {
            kind: StatementKind::Var(var),
            span: span.into_node(),
        };

        Ok((input, statement))
    }

    /*     pub fn parse_import(input: Span) -> Res<'a, Statement> {
        todo!()
    } */

    pub fn parse_expression(input: Span) -> Res<Statement> {
        let (input, (span, kind)) = consumed(Self::parse_expression_inner)(input)?;

        let statement = Self {
            kind,
            span: span.into_node(),
        };

        Ok((input, statement))
    }

    fn parse_expression_inner(input: Span) -> Res<StatementKind> {
        let (mut input, expr) = Expr::parse(input)?;
        if expr.is_block() {
            (input, _) = opt(preceded(ignorable, tag(";")))(input)?;
        } else {
            (input, _) = preceded(ignorable, expect(tag(";"), "Missing semicolon"))(input)?;
        };

        Ok((input, StatementKind::Expression(expr)))
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl Node for Statement {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum StatementKind {
    Expression(Expr),
    Var(Var),
    // Import(Import),
    #[default]
    Empty,
}

impl std::fmt::Display for StatementKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatementKind::Expression(e) => {
                e.fmt(f)?;

                if !e.is_block() {
                    f.write_str(";")?;
                }

                Ok(())
            }
            StatementKind::Var(v) => v.fmt(f),
            StatementKind::Empty => Ok(()),
            // StatementKind::Import(i) => i.fmt(f),
        }
    }
}
