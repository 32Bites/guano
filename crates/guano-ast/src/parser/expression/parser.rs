use guano_lexer::{escape_char::Token as EscapeToken, logos::Logos, Span, Token};

use crate::parser::{typing::Type, Parse, Parser};

use super::{
    display::Display,
    literal::Literal,
    operator::{
        BinaryOperator, ComparisonOperator, EqualityOperator, FactorOperator, TermOperator,
        UnaryOperator,
    },
    simplify::Simplify,
};

#[derive(Debug, Clone)]
pub enum Expression {
    Group(Box<Expression>),
    Literal(Literal),
    Variable(String),
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
    Cast {
        left: Box<Expression>,
        cast_to: Type,
    },
    Unary {
        operator: UnaryOperator,
        right: Box<Expression>,
    },
    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Format {
        format_string: String,
        arguments: Vec<Expression>,
    },
    Access {
        owner: Box<Expression>,
        accessed_value: Box<Expression>, // TODO: Recursive Access, save for later.
    },
    Index {
        owner: Box<Expression>,
        index: Box<Expression>,
    },
}

impl Parse for Expression {
    fn parse(parser: &mut Parser<impl Iterator<Item = (Token, Span)>>) -> Option<Expression> {
        Expression::equality(parser)
    }
}

impl Expression {
    pub fn display(&self) -> Display<'_> {
        Display::new(self, false)
    }

    pub fn display_grouped(&self) -> Display<'_> {
        Display::new(self, true)
    }

    pub fn equality(
        parser: &mut Parser<impl Iterator<Item = (Token, Span)>>,
    ) -> Option<Expression> {
        let mut left = Expression::comparison(parser)?;
        loop {
            let operator =
                if let Some((Token::Equals | Token::Exclamation, _)) = parser.lexer.peek() {
                    let equals = matches!(parser.lexer.next()?, (Token::Equals, _));
                    if let (Token::Equals, _) = parser.lexer.next()? {
                        if equals {
                            EqualityOperator::Equals
                        } else {
                            EqualityOperator::NotEquals
                        }
                    } else {
                        return None;
                    }
                } else {
                    parser.lexer.reset_peek();
                    break;
                };

            let right = Expression::comparison(parser)?;

            left = Expression::Binary {
                operator: operator.into(),
                left: Box::new(left),
                right: Box::new(right),
            }
            .simplify_binary();
        }

        Some(left)
    }

    pub fn comparison(
        parser: &mut Parser<impl Iterator<Item = (Token, Span)>>,
    ) -> Option<Expression> {
        let mut left = Expression::cast(parser)?;
        loop {
            let operator =
                if let Some((Token::GreaterThan | Token::LessThan, _)) = parser.lexer.peek() {
                    let (token, _) = parser.lexer.next()?;
                    let equality = parser
                        .lexer
                        .peek()
                        .map_or(false, |(t, _)| matches!(t, Token::Equals));

                    if equality {
                        parser.lexer.next();
                        match token {
                            Token::GreaterThan => ComparisonOperator::GreaterThanEquals,
                            Token::LessThan => ComparisonOperator::LessThanEquals,
                            _ => unreachable!(),
                        }
                    } else {
                        parser.lexer.reset_peek();
                        match token {
                            Token::GreaterThan => ComparisonOperator::GreaterThan,
                            Token::LessThan => ComparisonOperator::LessThan,
                            _ => unreachable!(),
                        }
                    }
                } else {
                    parser.lexer.reset_peek();
                    break;
                };

            let right = Expression::cast(parser)?;

            left = Expression::Binary {
                operator: operator.into(),
                left: Box::new(left),
                right: Box::new(right),
            }
            .simplify_binary();
        }

        Some(left)
    }

    pub fn cast(parser: &mut Parser<impl Iterator<Item = (Token, Span)>>) -> Option<Expression> {
        let mut left = Expression::term(parser)?;
        loop {
            if let Some((Token::KeyAs, _)) = parser.lexer.peek() {
                parser.lexer.next();

                let cast_type = Type::parse(parser)?;

                left = Expression::Cast {
                    left: Box::new(left),
                    cast_to: cast_type,
                }
                .simplify_cast();
            } else {
                parser.lexer.reset_peek();
                break;
            }
        }

        Some(left)
    }

    pub fn term(parser: &mut Parser<impl Iterator<Item = (Token, Span)>>) -> Option<Expression> {
        let mut left = Expression::factor(parser)?;
        loop {
            if let Some((token @ (Token::Plus | Token::Minus), _)) = parser.lexer.peek() {
                let operator = match token {
                    Token::Plus => TermOperator::Add,
                    Token::Minus => TermOperator::Subtract,
                    _ => unreachable!(),
                };

                parser.lexer.next();

                let right = Expression::factor(parser)?;

                left = Expression::Binary {
                    operator: operator.into(),
                    left: Box::new(left),
                    right: Box::new(right),
                }
                .simplify_binary();
            } else {
                parser.lexer.reset_peek();
                break;
            }
        }

        Some(left)
    }

    pub fn factor(parser: &mut Parser<impl Iterator<Item = (Token, Span)>>) -> Option<Expression> {
        let mut left = Expression::unary(parser)?;

        loop {
            if let Some((token @ (Token::Slash | Token::Asterisk), _)) = parser.lexer.peek() {
                let operator = match token {
                    Token::Slash => FactorOperator::Divide,
                    Token::Asterisk => FactorOperator::Multiply,
                    _ => unreachable!(),
                };

                parser.lexer.next();

                let right = Expression::unary(parser)?;

                left = Expression::Binary {
                    operator: operator.into(),
                    left: Box::new(left),
                    right: Box::new(right),
                }
                .simplify_binary();
            } else {
                parser.lexer.reset_peek();
                break;
            }
        }

        Some(left)
    }

    pub fn unary(parser: &mut Parser<impl Iterator<Item = (Token, Span)>>) -> Option<Expression> {
        if let Some((token, _)) = parser.lexer.peek() {
            match token {
                t @ (Token::Exclamation | Token::Minus) => {
                    let operator = match t {
                        Token::Exclamation => UnaryOperator::LogicalNegate,
                        Token::Minus => UnaryOperator::Negate,
                        _ => unreachable!(),
                    };

                    parser.lexer.next();

                    let expression = Expression::Unary {
                        operator,
                        right: Box::new(Expression::unary(parser)?),
                    };

                    Some(expression.simplify_unary())
                }

                _ => {
                    parser.lexer.reset_peek();
                    Expression::primary(parser)
                }
            }
        } else {
            None
        }
    }

    pub fn primary(parser: &mut Parser<impl Iterator<Item = (Token, Span)>>) -> Option<Expression> {
        if let Some((token, _)) = parser.lexer.peek() {
            let value = match token {
                Token::KeyNil => Some(Literal::Nil.to_expression()),
                Token::LitInteger(i) => {
                    let removed_underscores: String = i.chars().filter(|c| *c != '_').collect();
                    match removed_underscores.parse::<i64>() {
                        Ok(i) => Some(Literal::Integer(i).to_expression()),
                        Err(_) => match i.parse::<u64>() {
                            Ok(i) => Some(Literal::UnsignedInteger(i).to_expression()),
                            Err(_) => None,
                        },
                    }
                }
                Token::LitFloat(f) => {
                    let removed_underscores: String = f.chars().filter(|c| *c != '_').collect();
                    match removed_underscores.parse::<f64>() {
                        Ok(f) => Some(Literal::FloatingPoint(f).to_expression()),
                        Err(_) => todo!(),
                    }
                }
                Token::LitBool(b) => Some(Literal::Boolean(*b).to_expression()),
                Token::LitChar(c) => {
                    let mut escaper = EscapeToken::lexer(c);

                    match (escaper.next(), escaper.next()) {
                        (None, None) => None,
                        (None, Some(_)) => unreachable!(),
                        (Some(t), None) => Some(Literal::Character(t.char()?).to_expression()),
                        (Some(_), Some(_)) => None,
                    }
                }
                Token::LitString(s) => {
                    let string: Option<String> = EscapeToken::lexer(s).map(|t| t.char()).collect();

                    string.map(|s| Literal::String(s).to_expression())
                }
                Token::Identifier(i) => Some(Expression::Variable(i.clone())), // TODO: Handle function calls
                Token::OpenParen => {
                    parser.lexer.next();
                    let expr = Expression::parse(parser)?;
                    if let Some((Token::CloseParen, _)) = parser.lexer.peek() {
                        Some(Expression::Group(Box::new(expr)).simplify_group())
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if value.is_some() {
                parser.lexer.next();
            } else {
                parser.lexer.reset_peek();
            }

            value
        } else {
            None
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{Parse, Parser};

    use super::Expression;

    #[test]
    fn test() {
        let test = "!!true == true";

        let mut parser = <Parser>::from_source(test); // dafaq

        let expression = Expression::parse(&mut parser).unwrap();

        println!("Ungrouped: {}", expression.display());
        println!("Grouped: {}", expression.display_grouped());
        println!("Debug: {:?}", expression);
        println!("{:?}", parser.lexer.next());
    }
}
