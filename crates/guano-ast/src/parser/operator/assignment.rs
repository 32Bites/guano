use guano_lexer::Token;

use crate::parser::ParseContext;

use super::{Bitwise, Factor, Logical, ParseOperator, Term};

#[derive(Debug, Clone)]
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
            AssignmentOperator::Term(t) => t.as_ref(),
            AssignmentOperator::Factor(f) => f.as_ref(),
            AssignmentOperator::Bitwise(b) => b.as_ref(),
            AssignmentOperator::Logical(l) => l.as_ref(),
        }
    }
}

impl ParseOperator for AssignmentOperator {
    fn parse(context: &mut ParseContext) -> Option<Self> {
        Some(match &context.stream.peek_token::<3>() {
            [Some(Token::Equals), ..] => {
                context.stream.read::<1>();
                AssignmentOperator::Assign
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

                context.stream.read::<2>();

                operator
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
                context.stream.read::<3>();
                operator
            }
            _ => {
                context.stream.reset_peek();
                return None;
            }
        })
    }
}

impl std::fmt::Display for AssignmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}=", self.variant_string())
    }
}
