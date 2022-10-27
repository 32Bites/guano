use guano_lexer::Token;
use serde::{Serialize, Deserialize};

use crate::parser::{
    block::Block,
    error::{ParseError, ParseResult, ToParseResult},
    expression::Expression,
    Parse, ParseContext,
};

use super::StatementError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conditional {
    pub condition: Expression,
    pub block: Block,
    pub else_statement: Option<Else>,
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

impl Parse<StatementError> for Conditional {
    fn parse(context: &mut ParseContext) -> ParseResult<Self, StatementError> {
        match &context.stream.read::<1>()[0] {
            Some((token, span)) => match token {
                Token::KeyIf => {}
                _ => return Err(ParseError::unexpected_token(span.clone())),
            },
            None => return Err(ParseError::EndOfFile),
        }

        let condition = Expression::parse(context).to_parse_result()?;
        let block = Block::parse(context).to_parse_result()?;

        let else_statement = if let Some(Token::KeyElse) = context.stream.peek_token::<1>()[0] {
            Some(Else::parse(context)?)
        } else {
            context.stream.reset_peek();
            None
        };

        Ok(Conditional {
            condition,
            block,
            else_statement,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Else {
    Else(Block),
    ElseIf(Box<Conditional>),
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

impl Parse<StatementError> for Else {
    fn parse(context: &mut ParseContext) -> ParseResult<Self, StatementError> {
        match &context.stream.read::<1>()[0] {
            Some((token, span)) => match token {
                Token::KeyElse => match &context.stream.peek_token::<1>()[0] {
                    Some(token) => {
                        context.stream.reset_peek();
                        if let Token::OpenBrace = token {
                            Ok(Else::Else(Block::parse(context).to_parse_result()?))
                        } else {
                            Ok(Else::ElseIf(Box::new(Conditional::parse(context)?)))
                        }
                    }

                    None => Err(ParseError::EndOfFile),
                },
                _ => Err(ParseError::unexpected_token(span.clone())),
            },
            None => Err(ParseError::EndOfFile),
        }
    }
}
