use std::marker::PhantomData;

use super::{token_stream::Spanned, ParseContext};

pub mod assignment;
pub mod bitwise;
pub mod comparison;
pub mod factor;
pub mod logical;
pub mod term;
pub mod unary;

pub use assignment::AssignmentOperator as Assignment;
pub use bitwise::BitwiseOperator as Bitwise;
pub use comparison::ComparisonOperator as Comparison;
pub use factor::FactorOperator as Factor;
pub use logical::LogicalOperator as Logical;
pub use term::TermOperator as Term;
pub use unary::UnaryOperator as Unary;

pub trait ParseOperator<T = Self> {
    fn parse(context: &mut ParseContext) -> Option<Spanned<T>>;
}

pub trait Operator {
    type Str: AsRef<str>;

    fn name(&self) -> Self::Str;
    fn code(&self) -> Self::Str;

    fn display_name(&self) -> Display<'_, Self, Name>
    where
        Self: Sized,
    {
        Display {
            operator: self,
            _type: PhantomData,
        }
    }

    fn display_code(&self) -> Display<'_, Self, Code>
    where
        Self: Sized,
    {
        Display {
            operator: self,
            _type: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Name;
#[derive(Debug, Clone)]
pub struct Code;

#[derive(Debug, Clone)]
pub struct Display<'a, Op: Operator, T> {
    operator: &'a Op,
    _type: PhantomData<T>,
}

impl<Op: Operator> std::fmt::Display for Display<'_, Op, Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.operator.name().as_ref())
    }
}

impl<Op: Operator> std::fmt::Display for Display<'_, Op, Code> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.operator.code().as_ref())
    }
}
