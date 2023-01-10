pub mod ast;

#[cfg(test)]
mod tests {
    /*     use chumsky::Parser;

    use crate::{lexer::lexer, stream::ToStream, ast::parser};

    #[test]
    fn test() {
        let source = include_str!("../../../main.guano");

        let tokens = lexer().parse(source.to_stream()).unwrap();

        for spanned_token in &tokens {
            println!("{:#?}: {:?}", spanned_token.value(), &source[spanned_token.span()]);
        }

        let (tree, errors) = parser().parse_recovery_verbose(tokens.to_stream());

        //println!("{tree:#?}")
    } */
}
