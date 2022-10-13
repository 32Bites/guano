use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,
    LogicalNegate,
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            UnaryOperator::Negate => "-",
            UnaryOperator::LogicalNegate => "!",
        })
    }
}

impl Operator for UnaryOperator {
    fn precedence(&self) -> OperatorPrecedence {
        0.into()
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Comparison(ComparisonOperator),
    Term(TermOperator),
    Factor(FactorOperator),
    Equality(EqualityOperator),
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Comparison(c) => c.fmt(f),
            BinaryOperator::Term(t) => t.fmt(f),
            BinaryOperator::Factor(fc) => fc.fmt(f),
            BinaryOperator::Equality(e) => e.fmt(f),
        }
    }
}

impl From<ComparisonOperator> for BinaryOperator {
    fn from(c: ComparisonOperator) -> Self {
        Self::Comparison(c)
    }
}

impl From<TermOperator> for BinaryOperator {
    fn from(t: TermOperator) -> Self {
        Self::Term(t)
    }
}

impl From<FactorOperator> for BinaryOperator {
    fn from(f: FactorOperator) -> Self {
        Self::Factor(f)
    }
}

impl From<EqualityOperator> for BinaryOperator {
    fn from(e: EqualityOperator) -> Self {
        Self::Equality(e)
    }
}

impl Operator for BinaryOperator {
    fn precedence(&self) -> OperatorPrecedence {
        match self {
            BinaryOperator::Comparison(c) => c.precedence(),
            BinaryOperator::Term(t) => t.precedence(),
            BinaryOperator::Factor(f) => f.precedence(),
            BinaryOperator::Equality(e) => e.precedence(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EqualityOperator {
    Equals,
    NotEquals,
}

impl std::fmt::Display for EqualityOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            EqualityOperator::Equals => "==",
            EqualityOperator::NotEquals => "!=",
        })
    }
}

impl Operator for EqualityOperator {
    fn precedence(&self) -> OperatorPrecedence {
        4.into()
    }
}

#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
}

impl std::fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ComparisonOperator::GreaterThan => ">",
            ComparisonOperator::GreaterThanEquals => ">=",
            ComparisonOperator::LessThan => "<",
            ComparisonOperator::LessThanEquals => "<=",
        })
    }
}

impl Operator for ComparisonOperator {
    fn precedence(&self) -> OperatorPrecedence {
        4.into()
    }
}

#[derive(Debug, Clone)]
pub struct CastOperator;

impl Operator for CastOperator {
    fn precedence(&self) -> OperatorPrecedence {
        3.into()
    }
}

#[derive(Debug, Clone)]
pub enum TermOperator {
    Add,
    Subtract,
}

impl std::fmt::Display for TermOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TermOperator::Add => "+",
            TermOperator::Subtract => "-",
        })
    }
}

impl Operator for TermOperator {
    fn precedence(&self) -> OperatorPrecedence {
        2.into()
    }
}

#[derive(Debug, Clone)]
pub enum FactorOperator {
    Multiply,
    Divide,
}

impl std::fmt::Display for FactorOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            FactorOperator::Multiply => "*",
            FactorOperator::Divide => "/",
        })
    }
}

impl Operator for FactorOperator {
    fn precedence(&self) -> OperatorPrecedence {
        1.into()
    }
}

pub trait Operator {
    fn precedence(&self) -> OperatorPrecedence;
}

#[derive(Debug, PartialEq)]
pub struct OperatorPrecedence(usize);

impl From<usize> for OperatorPrecedence {
    fn from(u: usize) -> Self {
        OperatorPrecedence(u)
    }
}

impl PartialOrd for OperatorPrecedence {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0).map(|o| match o {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        })
    }
}
