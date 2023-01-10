use std::fmt::Write;

use crate::ast::prelude::*;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Block {
    statements: Vec<Statement>,
    end_expression: Option<Box<Expr>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    span: NodeSpan,
}

impl Block {
    pub fn statements(&self) -> &[Statement] {
        &self.statements
    }

    pub fn end_expression(&self) -> Option<&Expr> {
        self.end_expression.as_deref()
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Self> {
        let (input, (span, (statements, end_expression))) = consumed(delimited(
            tag("{"),
            padded(Self::parse_body),
            expect(tag("}"), "Expected a '}'"),
        ))(input)?;
        let block = Self {
            statements,
            end_expression,
            span: span.into_node(),
        };

        Ok((input, block))
    }

    fn parse_body(input: Span) -> Res<(Vec<Statement>, Option<Box<Expr>>)> {
        let (input, statements) = fold_many0(
            padded(Statement::parse),
            Vec::new,
            |mut statements, statement| {
                if !statement.is_empty() {
                    statements.push(statement);
                }

                statements
            },
        )(input)?;

        let (input, end_expression) = opt(map(padded(Expr::parse), Box::new))(input)?;

        Ok((input, (statements, end_expression)))
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = indenter::indented(f).with_str("\t");

        write!(f, "{{")?;

        for statement in &self.statements {
            write!(f, "\n{statement}")?;
        }

        if let Some(end_expression) = &self.end_expression {
            write!(f, "\n{end_expression}")?;
        }

        let has_final_line = !self.statements.is_empty() || self.end_expression.is_some();

        if has_final_line {
            write!(f, "\n")?;
        }

        write!(f, "}}")
    }
}

impl Node for Block {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}
