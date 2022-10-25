use super::ParseContext;

pub mod bitwise;
pub mod comparison;
pub mod factor;
pub mod logical;
pub mod term;
pub mod unary;

pub use bitwise::BitwiseOperator as Bitwise;
pub use comparison::ComparisonOperator as Comparison;
pub use factor::FactorOperator as Factor;
pub use logical::LogicalOperator as Logical;
pub use term::TermOperator as Term;
pub use unary::UnaryOperator as Unary;

#[derive(Debug)]
pub struct OperatorError;

impl std::fmt::Display for OperatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("operator error, this should be unreachable")
    }
}

impl std::error::Error for OperatorError {}

pub trait ParseOperator<T = Self> {
    fn parse(context: &mut ParseContext) -> Option<T>;
}
