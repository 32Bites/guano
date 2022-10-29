use guano_lexer::Token;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::parser::{
    block::BlockError,
    error::{ParseError, ParseResult, ToParseResult},
    expression::{Expression, ExpressionError},
    identifier::IdentifierError,
    operator::{Assignment, ParseOperator},
    token_stream::{MergeSpan, Span, Spannable, Spanned},
    Parse, ParseContext,
};

use super::{
    conditional::Conditional,
    for_loop::ForLoop,
    variable::{Variable, VariableError},
    while_loop::WhileLoop,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

impl Spannable for Statement {
    fn get_span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatementKind {
    Variable(Variable),
    Expression(Expression),
    Return(Option<Expression>),
    Assignment {
        left: Expression,
        operator: Spanned<Assignment>,
        right: Expression,
    },
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
    Conditional(Conditional),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StatementKind::Variable(v) => v.fmt(f)?,
            StatementKind::Expression(e) => e.fmt(f)?,
            StatementKind::Return(r) => {
                f.write_str("return")?;
                if let Some(r) = r {
                    write!(f, " {r}")?;
                }
            }
            StatementKind::Assignment {
                left,
                operator,
                right,
            } => write!(f, "{left} {operator} {right}")?,
            StatementKind::ForLoop(fl) => return fl.fmt(f),
            StatementKind::Conditional(c) => return c.fmt(f),
            StatementKind::WhileLoop(w) => return w.fmt(f),
        }

        f.write_str(";")
    }
}

impl Parse<StatementError, Result<Statement, Span>> for Statement {
    fn parse(context: &mut ParseContext) -> ParseResult<Result<Statement, Span>, StatementError> {
        let statement = match &context.stream.peek::<1>()[0] {
            Some((token, span)) => match token {
                Token::KeyVar | Token::KeyLet => Statement::variable(context),
                Token::KeyReturn => Statement::return_(context),
                Token::Semicolon => {
                    context.stream.read::<1>();

                    return Ok(Err(span.clone()));
                }
                Token::KeyIf => return Statement::if_(context).map(|s| Ok(s)),
                Token::KeyFor => return Statement::for_(context).map(|s| Ok(s)),
                Token::KeyWhile => return Statement::while_(context).map(|s| Ok(s)),
                _ => Statement::expression_or_assignment(context),
            },
            None => Err(ParseError::EndOfFile),
        }?;

        match &context.stream.read::<1>()[0] {
            Some((token, span)) => match token {
                Token::Semicolon => Ok(Ok(statement)),
                _ => Err(ParseError::unexpected_token(span.clone())),
            },
            None => Err(ParseError::EndOfFile),
        }
    }
}

impl Statement {
    fn variable(context: &mut ParseContext) -> ParseResult<Statement, StatementError> {
        let v = Variable::parse(context).to_parse_result()?;
        let kind = StatementKind::Variable(v.value);

        Ok(Statement { kind, span: v.span })
    }

    fn expression_or_assignment(
        context: &mut ParseContext,
    ) -> ParseResult<Statement, StatementError> {
        context.stream.reset_peek();
        let left = Expression::parse(context).to_parse_result()?;
        let mut final_kind = left.span.clone();

        if let Some(operator) = Assignment::parse(context) {
            final_kind = final_kind.merge(&operator.span);

            let right = Expression::parse(context).to_parse_result()?;
            final_kind = final_kind.merge(&right.span);

            let kind = StatementKind::Assignment {
                left,
                operator,
                right,
            };

            Ok(Statement {
                kind,
                span: final_kind,
            })
        } else {
            let kind = StatementKind::Expression(left);

            Ok(Statement {
                kind,
                span: final_kind,
            })
        }
    }

    fn if_(context: &mut ParseContext) -> ParseResult<Statement, StatementError> {
        let conditional = Conditional::parse(context)?;
        let kind = StatementKind::Conditional(conditional.value);

        Ok(Statement {
            kind,
            span: conditional.span,
        })
    }

    fn for_(context: &mut ParseContext) -> ParseResult<Statement, StatementError> {
        let floop = ForLoop::parse(context)?;
        let kind = StatementKind::ForLoop(floop.value);

        Ok(Statement {
            kind,
            span: floop.span,
        })
    }

    fn while_(context: &mut ParseContext) -> ParseResult<Statement, StatementError> {
        let wloop = WhileLoop::parse(context)?;
        let kind = StatementKind::WhileLoop(wloop.value);

        Ok(Statement {
            kind,
            span: wloop.span,
        })
    }

    fn return_(context: &mut ParseContext) -> ParseResult<Statement, StatementError> {
        match &context.stream.read::<1>()[0] {
            Some((Token::KeyReturn, span)) => {
                let mut final_span = span.clone();

                let expression = match context.stream.peek_token::<1>()[0] {
                    Some(Token::Semicolon) => {
                        context.stream.reset_peek();
                        None
                    }
                    _ => {
                        let expression = Expression::parse(context).to_parse_result()?;
                        final_span = final_span.merge(&expression.span);

                        Some(expression)
                    }
                };

                let kind = StatementKind::Return(expression);

                Ok(Statement {
                    kind,
                    span: final_span,
                })
            }
            Some((_, span)) => Err(ParseError::unexpected_token(span.clone())),
            None => Err(ParseError::EndOfFile),
        }
    }
}

#[derive(Debug, Error)]
pub enum StatementError {
    #[error("{0}")]
    VariableError(#[from] VariableError),
    #[error("{0}")]
    IdentifierError(#[from] IdentifierError),
    #[error("{0}")]
    ExpressionError(#[from] ExpressionError),
    #[error("{0}")]
    BlockError(#[from] BlockError),
    #[error("expected semicolon")]
    MissingSemicolon,
}
