pub mod expression;
pub mod literal;
pub mod operator;
pub mod declaration;
pub mod typing;
pub mod statement;
pub mod block;
pub mod source_file;
pub mod span;
pub mod parser;

pub use parser::*;

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn test_parser() {
        let source = include_str!("../../../../example.gno");

        let mut parser = Parser::new();
        let result = parser.file("example.gno", source);

        match result {
            Ok(file_id) => println!("{:#?}", parser.syntax_tree(file_id).unwrap()),
            Err(error) => println!("Err: {error}"),
        }
    }
}