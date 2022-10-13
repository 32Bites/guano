use std::mem;

use num::ToPrimitive;

use crate::parser::typing::Type;

use super::{
    literal::Literal,
    operator::{
        BinaryOperator, ComparisonOperator, EqualityOperator, FactorOperator, TermOperator,
        UnaryOperator,
    },
    parser::Expression,
};

pub trait Simplify: Sized {
    fn simplify_group(self) -> Self;
    fn simplify_unary(self) -> Self;
    fn simplify_cast(self) -> Self;
    fn simplify_binary(self) -> Self;
}

impl Simplify for Expression {
    fn simplify_binary(mut self) -> Self {
        if let Self::Binary {
            operator,
            left,
            right,
        } = &self
        {
            if let (Expression::Literal(left), Expression::Literal(right)) =
                (*left.clone(), *right.clone())
            {
                let result = match operator {
                    BinaryOperator::Comparison(c) => Some(Literal::Boolean(match c {
                        ComparisonOperator::GreaterThan => left > right,
                        ComparisonOperator::GreaterThanEquals => left >= right,
                        ComparisonOperator::LessThan => left < right,
                        ComparisonOperator::LessThanEquals => left <= right,
                    })),
                    BinaryOperator::Term(t) => match t {
                        TermOperator::Add => left + right,
                        TermOperator::Subtract => left - right,
                    },
                    BinaryOperator::Factor(f) => match f {
                        FactorOperator::Multiply => left * right,
                        FactorOperator::Divide => left / right,
                    },
                    BinaryOperator::Equality(e) => Some(Literal::Boolean(match e {
                        EqualityOperator::Equals => left == right,
                        EqualityOperator::NotEquals => left != right,
                    })),
                };

                if let Some(new) = result {
                    let _ = mem::replace(&mut self, new.to_expression());
                }
            }
        }

        self
    }

    fn simplify_cast(mut self) -> Self {
        if let Self::Cast { left, cast_to } = &self {
            if let Self::Literal(literal) = &**left {
                let result = match cast_to {
                    Type::String => match literal {
                        Literal::String(s) => Some(Literal::String(s.clone())),
                        Literal::Character(c) => Some(Literal::String(format!("{c}"))),
                        Literal::Integer(i) => Some(Literal::String(format!("{i}"))),
                        Literal::UnsignedInteger(u) => Some(Literal::String(format!("{u}"))),
                        Literal::FloatingPoint(f) => Some(Literal::String(format!("{f}"))),
                        Literal::Boolean(b) => Some(Literal::String(format!("{b}"))),
                        Literal::Nil => Some(Literal::String("nil".into())),
                    },
                    Type::Character => match literal {
                        Literal::Character(c) => Some(Literal::Character(c.clone())),
                        Literal::Integer(i) => {
                            if let Some(c) = i.to_u32().and_then(|u| char::from_u32(u)) {
                                Some(Literal::Character(c))
                            } else {
                                None
                            }
                        }
                        Literal::UnsignedInteger(u) => {
                            if let Some(c) = u.to_u32().and_then(|u| char::from_u32(u)) {
                                Some(Literal::Character(c))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                    Type::Integer => match literal {
                        Literal::Character(c) => {
                            if let Some(i) = (*c as u32).to_i64() {
                                Some(Literal::Integer(i))
                            } else {
                                None
                            }
                        }
                        Literal::Boolean(b) => Some(Literal::Integer(*b as i64)),
                        Literal::Nil => Some(Literal::Integer(0)),
                        l => {
                            if let Some(n) = l.primitive().and_then(|p| p.to_i64()) {
                                Some(Literal::Integer(n))
                            } else {
                                None
                            }
                        }
                    },
                    Type::UnsignedInteger => match literal {
                        Literal::Character(c) => Some(Literal::UnsignedInteger(*c as u64)),
                        Literal::Boolean(b) => Some(Literal::UnsignedInteger(*b as u64)),
                        Literal::Nil => Some(Literal::UnsignedInteger(0)),
                        l => {
                            if let Some(n) = l.primitive().and_then(|p| p.to_u64()) {
                                Some(Literal::UnsignedInteger(n))
                            } else {
                                None
                            }
                        }
                    },
                    Type::Boolean => match literal {
                        Literal::String(s) => Some(Literal::Boolean(s.len() > 0)),
                        Literal::Character(_) => Some(Literal::Boolean(true)),
                        Literal::Integer(i) => Some(Literal::Boolean(*i != 0)),
                        Literal::UnsignedInteger(u) => Some(Literal::Boolean(*u > 0)),
                        Literal::FloatingPoint(f) => Some(Literal::Boolean(*f != 0.0)),
                        Literal::Boolean(b) => Some(Literal::Boolean(*b)),
                        Literal::Nil => Some(Literal::Boolean(false)),
                    },
                    Type::FloatingPoint => match literal {
                        Literal::Nil => Some(Literal::FloatingPoint(0.0)),
                        l => {
                            if let Some(n) = l.primitive().and_then(|p| p.to_f64()) {
                                Some(Literal::FloatingPoint(n))
                            } else {
                                None
                            }
                        }
                    },
                    Type::Custom(_) => None,
                };

                if let Some(new) = result {
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
                        Literal::Integer(i) => Some(Literal::Integer(-i)),
                        Literal::UnsignedInteger(u) => {
                            if let Some(i) = u.to_i64() {
                                Some(Literal::Integer(-i))
                            } else {
                                None
                            }
                        }
                        Literal::FloatingPoint(f) => Some(Literal::FloatingPoint(-f)),
                        _ => None,
                    },
                    Expression::Binary {
                        operator: _,
                        left: _,
                        right: _,
                    } => None,
                    _ => None,
                },
                UnaryOperator::LogicalNegate => match &**right {
                    Expression::Literal(literal) => match literal {
                        Literal::Integer(i) => Some(Literal::Integer(!i)),
                        Literal::UnsignedInteger(u) => Some(Literal::UnsignedInteger(!u)),
                        Literal::Boolean(b) => Some(Literal::Boolean(!b)),
                        _ => None,
                    },
                    Expression::Binary {
                        operator: _,
                        left: _,
                        right: _,
                    } => None,
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
}
