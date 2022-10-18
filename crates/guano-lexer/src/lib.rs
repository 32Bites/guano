pub mod escape_char;

mod spanned_lexer;
mod token;

pub use logos;
pub use spanned_lexer::*;
pub use token::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test_identifier() {}

    #[test]
    fn test_single_comment() {}

    #[test]
    fn test_multi_comment() {}
}
