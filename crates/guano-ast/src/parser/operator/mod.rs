use super::ParseContext;

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
    fn parse(context: &mut ParseContext) -> Option<T>;
}
