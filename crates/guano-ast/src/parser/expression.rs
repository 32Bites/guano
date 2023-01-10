use super::{
    literal::{Literal, LiteralError},
    operator::{BinaryOperator, Operator, UnaryOperator},
    parser::Rule,
    typing::Type, span::{SpanStr, Span, IntoSpan},
};
use owning_ref::RcRef;
use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::{Assoc, Op, PrattParser},
};
use thiserror::Error;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        PrattParser::new()
            .op(Op::infix(Rule::logical, Assoc::Left))
            .op(Op::infix(Rule::relational, Assoc::Left))
            .op(Op::infix(Rule::bitwise, Assoc::Left))
            .op(Op::infix(Rule::term, Assoc::Left))
            .op(Op::infix(Rule::factor, Assoc::Left))
            .op(Op::postfix(Rule::cast))
            .op(Op::prefix(Rule::negate) | Op::prefix(Rule::not))
            .op(Op::postfix(Rule::call) | Op::postfix(Rule::index) | Op::postfix(Rule::property))
    };
}

#[derive(Clone)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl Expression {
    pub fn parse(pairs: Pairs<'_, Rule>, input: RcRef<str>) -> Result<Self, ExpressionError> {
        PRATT_PARSER
            .map_primary(
                |primary: Pair<'_, Rule>| -> Result<Expression, ExpressionError> {
                    let span = primary.as_span().into_span(input.clone());
                    let kind = match primary.as_rule() {
                        Rule::literal
                        | Rule::list_literal
                        | Rule::text_literal
                        | Rule::nil_literal
                        | Rule::tuple_literal
                        | Rule::number_literal
                        | Rule::string_literal
                        | Rule::boolean_literal
                        | Rule::character_literal
                        | Rule::collection_literal => {
                            let literal = Literal::parse(primary, input.clone())?;
                            ExpressionKind::Literal(literal)
                        }
                        Rule::grouped => {
                            let expression =
                                Self::parse(primary.into_inner().next().unwrap().into_inner(), input.clone())?;

                            ExpressionKind::Grouped(Box::new(expression))
                        }
                        Rule::this_keyword => ExpressionKind::This,
                        Rule::identifier => ExpressionKind::Variable(primary.into_span_str(input.clone())),
                        r => panic!("{r:?} - {:?}", primary.as_str()),
                    };

                    Ok(Expression { kind, span })
                },
            )
            .map_prefix(|operator, rhs| {
                let rhs = rhs?;
                let operator = Operator::<UnaryOperator>::parse(operator, input.clone());

                let span = &operator.span + &rhs.span;
                Ok(Expression {
                    kind: ExpressionKind::Unary {
                        operator,
                        right: Box::new(rhs),
                    },
                    span,
                })
            })
            .map_postfix(|lhs, operator| {
                let lhs = lhs?;
                let span = &lhs.span + &operator.as_span().into_span(input.clone());

                let kind = match operator.as_rule() {
                    Rule::call => {
                        if !lhs.kind.can_access() {
                            return Err(ExpressionError::CannotNonPrimary(
                                "call",
                                lhs.span.as_str().to_string(),
                            ));
                        }
                        let mut arguments = vec![];

                        if let Some(list_inner) = operator.into_inner().next() {
                            for argument in list_inner.into_inner() {
                                let expression = Expression::parse(
                                    argument.into_inner().next().unwrap().into_inner(),
                                    input.clone()
                                )?;
                                arguments.push(expression);
                            }
                        }

                        ExpressionKind::Call {
                            function: Box::new(lhs),
                            arguments,
                        }
                    }
                    Rule::index => {
                        if !lhs.kind.can_access() {
                            return Err(ExpressionError::CannotNonPrimary(
                                "index",
                                lhs.span.as_str().to_string(),
                            ));
                        }
                        let index =
                            Expression::parse(operator.into_inner().next().unwrap().into_inner(), input.clone())?;

                        ExpressionKind::Index {
                            parent: Box::new(lhs),
                            index_by: Box::new(index),
                        }
                    }
                    Rule::property => {
                        if !lhs.kind.can_access() {
                            return Err(ExpressionError::CannotNonPrimary(
                                "access property",
                                lhs.span.as_str().to_string(),
                            ));
                        }
                        let property = operator.into_inner().next().unwrap().into_span_str(input.clone());
                        ExpressionKind::Property {
                            parent: Box::new(lhs),
                            property,
                        }
                    }
                    Rule::cast => {
                        let ty = Type::parse(operator.into_inner().next().unwrap(), input.clone());

                        ExpressionKind::Cast {
                            value: Box::new(lhs),
                            to: ty,
                        }
                    }
                    _ => unreachable!(),
                };

                Ok(Expression { span, kind })
            })
            .map_infix(|lhs, operator, rhs| match operator.as_rule() {
                Rule::factor | Rule::term | Rule::bitwise | Rule::relational | Rule::logical => {
                    let left = lhs?;
                    let right = rhs?;

                    let operator = Operator::<BinaryOperator>::parse(operator.into_inner().next().unwrap(), input.clone());

                    let span = &left.span + &right.span;
                    let kind = ExpressionKind::Binary {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    };

                    Ok(Expression { span, kind })
                }
                _ => unreachable!(),
            })
            .parse(pairs)
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionKind {
    /// Primary
    Grouped(Box<Expression>),
    Literal(Literal),
    Variable(SpanStr),
    This,

    // Prefix
    Unary {
        operator: Operator<UnaryOperator>,
        right: Box<Expression>,
    },

    // Postfix
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Index {
        parent: Box<Expression>,
        index_by: Box<Expression>,
    },
    Property {
        parent: Box<Expression>,
        property: SpanStr,
    },

    // Infix
    Cast {
        value: Box<Expression>,
        to: Type,
    },
    Binary {
        left: Box<Expression>,
        operator: Operator<BinaryOperator>,
        right: Box<Expression>,
    },
}

impl ExpressionKind {
    pub fn can_access(&self) -> bool {
        match self {
            ExpressionKind::Grouped(_) => true,
            ExpressionKind::Literal(_) => true,
            ExpressionKind::Variable(_) => true,
            ExpressionKind::This => true,
            ExpressionKind::Call {
                function: _,
                arguments: _,
            } => true,
            ExpressionKind::Index {
                parent: _,
                index_by: _,
            } => true,
            ExpressionKind::Property {
                parent: _,
                property: _,
            } => true,

            ExpressionKind::Unary {
                operator: _,
                right: _,
            } => false,
            ExpressionKind::Cast { value: _, to: _ } => false,
            ExpressionKind::Binary {
                left: _,
                operator: _,
                right: _,
            } => false,
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum ExpressionError {
    #[error("{0}")]
    LiteralError(#[from] LiteralError),
    #[error("cannot apply prefix operator to a type")]
    PrefixOperatorType,
    #[error("cannot apply postfix operator to a type (yet)")]
    PostfixOperatorType,
    #[error("invalid expression {0:?}")]
    InvalidExpression(String),
    #[error("invalid type {0:?}")]
    InvalidType(String),
    #[error("cannot {0} on non-primary expression {1}")]
    CannotNonPrimary(&'static str, String),
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::parser::InternalParser;
    use super::super::Rule;
    #[test]
    fn test_expression() {
        let expression = "(5 + 6) << 1 >> 2 + ree(1)[0][1].ree.ree().mee(1, 2) as []uint + (1, 2) - (1 + 4,) + (1,)";
        // let expression = "(1 as int)()";

        let mut res = InternalParser::parse(Rule::expression, expression).unwrap();
        let _next = res.next().unwrap();

        // let ty = Expression::parse(next.into_inner()).unwrap();
        // println!("{ty:#?}");
    }
}
