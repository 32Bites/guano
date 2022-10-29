use guano_lexer::Token;
use serde::{Deserialize, Serialize};

use crate::parser::{
    token_stream::{Spanned, ToSpanned},
    ParseContext,
};

use super::{Bitwise, Factor, Logical, Operator, ParseOperator, Term};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssignmentOperator {
    Assign,
    Term(Term),
    Factor(Factor),
    Bitwise(Bitwise),
    Logical(Logical),
}

impl AssignmentOperator {
    fn variant_string(&self) -> &str {
        match self {
            AssignmentOperator::Assign => "",
            AssignmentOperator::Term(t) => t.code(),
            AssignmentOperator::Factor(f) => f.code(),
            AssignmentOperator::Bitwise(b) => b.code(),
            AssignmentOperator::Logical(l) => l.code(),
        }
    }
}

impl ParseOperator for AssignmentOperator {
    fn parse(context: &mut ParseContext) -> Option<Spanned<Self>> {
        Some(match &context.stream.peek_token::<3>() {
            [Some(Token::Equals), ..] => {
                AssignmentOperator::Assign.to_spanned(context.stream.read_span::<1>()[0].clone()?)
            }
            [Some(token), Some(Token::Equals), _] => {
                let operator = match token {
                    Token::Plus => AssignmentOperator::Term(Term::Add),
                    Token::Minus => AssignmentOperator::Term(Term::Subtract),
                    Token::Asterisk => AssignmentOperator::Factor(Factor::Multiply),
                    Token::Slash => AssignmentOperator::Factor(Factor::Divide),
                    Token::Caret => AssignmentOperator::Bitwise(Bitwise::Xor),
                    Token::Pipe => AssignmentOperator::Bitwise(Bitwise::Or),
                    Token::Ampersand => AssignmentOperator::Bitwise(Bitwise::And),
                    _ => {
                        context.stream.reset_peek();
                        return None;
                    }
                };

                operator.to_spanned(context.stream.read_span_combined::<2>()?)
            }
            [Some(first), Some(second), Some(Token::Equals)] => {
                let operator = match (first, second) {
                    (Token::GreaterThan, Token::GreaterThan) => {
                        AssignmentOperator::Bitwise(Bitwise::ShiftRight)
                    }
                    (Token::LessThan, Token::LessThan) => {
                        AssignmentOperator::Bitwise(Bitwise::ShiftLeft)
                    }
                    (Token::Ampersand, Token::Ampersand) => {
                        AssignmentOperator::Logical(Logical::And)
                    }
                    (Token::Pipe, Token::Pipe) => AssignmentOperator::Logical(Logical::Or),
                    _ => {
                        context.stream.reset_peek();
                        return None;
                    }
                };
                operator.to_spanned(context.stream.read_span_combined::<3>()?)
            }
            _ => {
                context.stream.reset_peek();
                return None;
            }
        })
    }
}

impl Operator for AssignmentOperator {
    type Str = String;

    fn name(&self) -> Self::Str {
        format!(
            "{} assignment",
            match self {
                AssignmentOperator::Assign => "",
                AssignmentOperator::Term(t) => t.name(),
                AssignmentOperator::Factor(f) => f.name(),
                AssignmentOperator::Bitwise(b) => b.name(),
                AssignmentOperator::Logical(l) => l.name(),
            }
        )
    }

    fn code(&self) -> Self::Str {
        format!(
            "{}=",
            match self {
                AssignmentOperator::Assign => "",
                AssignmentOperator::Term(t) => t.code(),
                AssignmentOperator::Factor(f) => f.code(),
                AssignmentOperator::Bitwise(b) => b.code(),
                AssignmentOperator::Logical(l) => l.code(),
            }
        )
    }
}

impl std::fmt::Display for AssignmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}=", self.variant_string())
    }
}
