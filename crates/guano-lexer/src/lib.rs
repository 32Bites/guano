pub mod escape_char;

mod token;
mod spanned_lexer;

pub use token::*;
pub use spanned_lexer::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test_identifier() {}

    #[test]
    fn test_single_comment() {}

    #[test]
    fn test_multi_comment() {}
}
