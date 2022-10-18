use std::mem;

use super::{
    literal::Literal,
    operator::{ComparisonOperator, EqualityOperator, FactorOperator, TermOperator, UnaryOperator},
    parser::Expression,
};

pub trait Simplify: Sized {
    fn simplify_group(self) -> Self;
    fn simplify_unary(self) -> Self;
    fn simplify_cast(self) -> Self;
    fn simplify_comparison(self) -> Self;
    fn simplify_equality(self) -> Self;
    fn simplify_factor(self) -> Self;
    fn simplify_term(self) -> Self;
}

impl Simplify for Expression {
    fn simplify_cast(mut self) -> Self {
        if let Self::Cast { left, cast_to } = &self {
            if let Self::Literal(literal) = &**left {
                if let Some(new) = literal.cast(cast_to) {
                    let _ = mem::replace(&mut self, new.to_expression());
                }
            }
        }

        self
    }

    fn simplify_unary(mut self) -> Self {
        if let Self::Unary { operator, right } = &self {
            let to_replace = match operator {
                UnaryOperator::Negate => match &**right {
                    Expression::Literal(literal) => match literal {
                        Literal::FloatingPoint(f) => Some(Literal::FloatingPoint(-f)),
                        Literal::Integer(i) => Some(Literal::Integer(-i)),
                        _ => None,
                    },
                    _ => None,
                },
                UnaryOperator::LogicalNegate => match &**right {
                    Expression::Literal(literal) => match literal {
                        Literal::Boolean(b) => Some(Literal::Boolean(!b)),
                        Literal::Integer(i) => Some(Literal::Integer(!i)),
                        _ => None,
                    },
                    _ => None,
                },
            };

            if let Some(new) = to_replace {
                let _ = mem::replace(&mut self, new.to_expression());
            }
        }

        self
    }

    fn simplify_group(mut self) -> Self {
        if let Self::Group(expression) = &self {
            match &**expression {
                e @ (Expression::Group(_) | Expression::Literal(_) | Expression::Variable(_)) => {
                    let e = e.clone();
                    let _ = mem::replace(&mut self, e);
                }
                _ => {}
            }
        }

        self
    }

    fn simplify_comparison(mut self) -> Self {
        if let Self::Comparison {
            operator,
            left,
            right,
        } = &self
        {
            if let (Expression::Literal(left), Expression::Literal(right)) = (&**left, &**right) {
                let result = match operator {
                    ComparisonOperator::GreaterThan => left.gt(right),
                    ComparisonOperator::GreaterThanEquals => left.ge(right),
                    ComparisonOperator::LessThan => left.lt(right),
                    ComparisonOperator::LessThanEquals => left.le(right),
                }
                .map(|b| Literal::Boolean(b));

                if let Some(new) = result {
                    let _ = mem::replace(&mut self, new.to_expression());
                }
            }
        }

        self
    }

    fn simplify_equality(mut self) -> Self {
        if let Self::Equality {
            operator,
            left,
            right,
        } = &self
        {
            if let (Expression::Literal(left), Expression::Literal(right)) = (&**left, &**right) {
                let result = match operator {
                    EqualityOperator::Equals => left.eq(right),
                    EqualityOperator::NotEquals => left.ne(right),
                }
                .map(|b| Literal::Boolean(b));

                if let Some(new) = result {
                    let _ = mem::replace(&mut self, new.to_expression());
                }
            }
        }

        self
    }

    fn simplify_factor(mut self) -> Self {
        if let Self::Factor {
            operator,
            left,
            right,
        } = &self
        {
            if let (Expression::Literal(left), Expression::Literal(right)) = (&**left, &**right) {
                let result = match operator {
                    FactorOperator::Multiply => left.mul(right),
                    FactorOperator::Divide => left.div(right),
                };

                if let Some(new) = result {
                    let _ = mem::replace(&mut self, new.to_expression());
                }
            }
        }

        self
    }

    fn simplify_term(mut self) -> Self {
        if let Self::Term {
            operator,
            left,
            right,
        } = &self
        {
            if let (Expression::Literal(left), Expression::Literal(right)) = (&**left, &**right) {
                let result = match operator {
                    TermOperator::Add => left.add(right),
                    TermOperator::Subtract => left.sub(right),
                };

                if let Some(new) = result {
                    let _ = mem::replace(&mut self, new.to_expression());
                }
            }
        }

        self
    }
}
