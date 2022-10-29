use guano_lexer::Token;
use serde::{Deserialize, Serialize};

use crate::parser::{
    block::Block,
    error::{ParseError, ParseResult, ToParseResult},
    expression::Expression,
    token_stream::{MergeSpan, Spanned, ToSpanned},
    Parse, ParseContext,
};

use super::StatementError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conditional {
    pub condition: Expression,
    pub block: Block,
    pub else_statement: Option<Spanned<Else>>,
}

impl std::fmt::Display for Conditional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {} {}", self.condition, self.block)?;
        if let Some(else_statement) = &self.else_statement {
            write!(f, " {else_statement}")
        } else {
            Ok(())
        }
    }
}

impl Parse<StatementError, Spanned<Conditional>> for Conditional {
    fn parse(context: &mut ParseContext) -> ParseResult<Spanned<Conditional>, StatementError> {
        let mut final_span = match &context.stream.read::<1>()[0] {
            Some((token, span)) => match token {
                Token::KeyIf => span.clone(),
                _ => return Err(ParseError::unexpected_token(span.clone())),
            },
            None => return Err(ParseError::EndOfFile),
        };

        let condition = Expression::parse(context).to_parse_result()?;
        final_span = final_span.merge(&condition.span);

        let block = Block::parse(context).to_parse_result()?;
        final_span = final_span.merge(&block.span);

        let else_statement = if let Some((Token::KeyElse, span)) = &context.stream.peek::<1>()[0] {
            final_span = final_span.merge(span);

            let else_statement = Else::parse(context)?;
            final_span = final_span.merge(&else_statement.span);

            Some(else_statement)
        } else {
            context.stream.reset_peek();
            None
        };

        Ok(Conditional {
            condition,
            block,
            else_statement,
        }
        .to_spanned(final_span))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Else {
    Else(Block),
    ElseIf(Box<Spanned<Conditional>>),
}

impl std::fmt::Display for Else {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("else ")?;
        match self {
            Else::Else(b) => b.fmt(f),
            Else::ElseIf(c) => c.fmt(f),
        }
    }
}

impl Parse<StatementError, Spanned<Else>> for Else {
    fn parse(context: &mut ParseContext) -> ParseResult<Spanned<Else>, StatementError> {
        match &context.stream.read::<1>()[0] {
            Some((token, span)) => match token {
                Token::KeyElse => {
                    let mut final_span = span.clone();
                    match &context.stream.peek_token::<1>()[0] {
                        Some(token) => {
                            context.stream.reset_peek();
                            let code = if let Token::OpenBrace = token {
                                let block = Block::parse(context).to_parse_result()?;
                                final_span = final_span.merge(&block.span);

                                Else::Else(block)
                            } else {
                                let conditional = Conditional::parse(context)?;
                                final_span = final_span.merge(&conditional.span);
                                Else::ElseIf(Box::new(conditional))
                            };

                            Ok(code.to_spanned(final_span))
                        }

                        None => Err(ParseError::EndOfFile),
                    }
                }
                _ => Err(ParseError::unexpected_token(span.clone())),
            },
            None => Err(ParseError::EndOfFile),
        }
    }
}
