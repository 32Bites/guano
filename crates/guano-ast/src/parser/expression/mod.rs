pub mod display;
mod parser;
mod simplify;

pub use parser::*;

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    use super::Expression;

    #[test]
    fn test_expression() {
        let test = "(5 + 6) << 1 >> 2 + ree(1)[0][1].ree.ree().mee(1, 2) as []uint + (1, 2) - (1 + 4,) + (1,)";

        let mut parser = Parser::new(false);
        let (_, result) = parser.parse_file::<Expression, _, _>("", test);

        match result {
            Ok(expression) => {
                println!("Ungrouped: {}", expression.display());
                println!("Grouped: {}", expression.display_grouped());
                println!("Debug: {expression:#?}");
            }
            Err(error) => println!("An error occurred when parsing the expression: {error}"),
        }
    }
}
