use std::mem;

use super::{
    literal::Literal,
    operator::{ComparisonOperator, EqualityOperator, FactorOperator, TermOperator, UnaryOperator, BitwiseOperator, LogicalOperator},
    parser::Expression,
};


/// Parse-time expression simplification, what is it?
/// Well, depending on the type of expression, and the type(s) of it's child(ren),
/// it may be reduced into a simpler expression.
/// 
/// Example: `2 + 2` is equivalent to `4`, so `2 + 2` will be replaced with the expression `4`.
/// However, `2 + a`, which includes a variable, cannot be simplified at parse time (as of yet),
/// so it will remain `2 + a`.
pub trait Simplify: Sized {
    fn simplify_group(self, should_simplify: bool) -> Self;
    fn simplify_unary(self, should_simplify: bool) -> Self;
    fn simplify_cast(self, should_simplify: bool) -> Self;
    fn simplify_comparison(self, should_simplify: bool) -> Self;
    fn simplify_equality(self, should_simplify: bool) -> Self;
    fn simplify_factor(self, should_simplify: bool) -> Self;
    fn simplify_term(self, should_simplify: bool) -> Self;
    fn simplify_bitwise(self, should_simplify: bool) -> Self;
    fn simplify_logical(self, should_simplify: bool) -> Self;
}

impl Simplify for Expression {
    fn simplify_cast(mut self, should_simplify: bool) -> Self {
        if should_simplify {
            if let Self::Cast { left, cast_to } = &self {
                if let Self::Literal(literal) = &**left {
                    if let Some(new) = literal.cast(cast_to) {
                        let _ = mem::replace(&mut self, new.to_expression());
                    }
                }
            }
        }

        self
    }

    fn simplify_unary(mut self, should_simplify: bool) -> Self {
        if should_simplify {
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
        }

        self
    }

    fn simplify_group(mut self, should_simplify: bool) -> Self {
        if should_simplify {
            if let Self::Group(expression) = &self {
                match &**expression {
                    e @ (Expression::Group(_) | Expression::Literal(_) | Expression::Variable(_)) => {
                        let e = e.clone();
                        let _ = mem::replace(&mut self, e);
                    }
                    _ => {}
                }
            }
        }

        self
    }

    fn simplify_comparison(mut self, should_simplify: bool) -> Self {
        if should_simplify {
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
        }

        self
    }

    fn simplify_equality(mut self, should_simplify: bool) -> Self {
        if should_simplify {
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
        }

        self
    }

    fn simplify_factor(mut self, should_simplify: bool) -> Self {
        if should_simplify {
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
        }

        self
    }

    fn simplify_term(mut self, should_simplify: bool) -> Self {
        if should_simplify {
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
        }

        self
    }

    fn simplify_bitwise(mut self, should_simplify: bool) -> Self {
        if should_simplify {
            if let Self::Bitwise { operator, left, right } = &self {
                if let (Expression::Literal(left), Expression::Literal(right)) = (&**left, &**right) {
                    let result = match operator {
                        BitwiseOperator::ShiftLeft => left.bs_left(right),
                        BitwiseOperator::ShiftRight => left.bs_right(right),
                        BitwiseOperator::Or => left.b_or(right),
                        BitwiseOperator::Xor => left.b_xor(right),
                        BitwiseOperator::And => left.b_and(right),
                    };
    
                    if let Some(new) = result {
                        let _ = mem::replace(&mut self, new.to_expression());
                    }
                }
            }
        }

        self
    }

    fn simplify_logical(mut self, should_simplify: bool) -> Self {
        if should_simplify {
            if let Self::Logical { operator, left, right } = &self {
                if let (Expression::Literal(left), Expression::Literal(right)) = (&**left, &**right) {
                    let result = match operator {
                        LogicalOperator::And => left.l_and(right),
                        LogicalOperator::Or => left.l_or(right),
                    };
    
                    if let Some(new) = result {
                        let _ = mem::replace(&mut self, new.to_expression());
                    }
                }
            }
        }
        
        self
    }
}
