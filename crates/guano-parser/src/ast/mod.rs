pub mod parser;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]

#[cfg(test)]
mod test {
    use crate::ast::parser::{combinators::Combinators, Parser, ParserContext};

    #[test]
    fn test_parser() {
        let source = "iden this not sefojseijfoisjefoij kasfjhbfjhb      ";

        let mut parser = ParserContext::new_str(source).unwrap();

        let results = ParserContext::raw_identifier
            .prefixed(ParserContext::eat_whitespace)
            .repeated()
            .parse(&mut parser)
            .unwrap();

        for res in results {
            println!("{res}");
        }

        /*         match parser.keyword() {
            Ok(output) => println!("Output: {output:?}"),
            Err(error) => println!("Error: {error}"),
        } */

        println!("Remaining: {:?}", parser.remaining());
    }
}
