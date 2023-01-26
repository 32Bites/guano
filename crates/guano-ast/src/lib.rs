pub mod parsing;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]

#[cfg(test)]
mod test {

    use guano_common::rowan::ast::AstNode;
    use guano_syntax::{
        nodes::{Literal, Path},
        SyntaxNode,
    };

    use crate::parsing::{
        combinators::Combinators,
        parsers::{
            expression::primary::literal::literal, ignorable::eat_ignorable, symbols::path::path,
        },
        ParseContext, Parser,
    };

    #[test]
    fn test_path() {
        let source = "hello::my::name::iss:: fesfsefk \n:: hello";

        let mut context = ParseContext::new(source);
        let parsed_path = path.parse(&mut context).unwrap();

        println!("{parsed_path:?}");
        println!("Remaining: {:?}", context.remaining());

        let root = SyntaxNode::new_root(parsed_path.into_node().unwrap());
        println!("SyntaxNode: {root:#?}");
        let path = Path::cast(root).unwrap();
        println!("Path: {:?}", path.to_string());

        /*         for segment in path.segments() {
            println!("Segment Name: {}", segment.segment().unwrap());
        } */
    }

    #[test]
    fn test_literal() {
        let source = "1.0000 500 nil nan inf 0b010101 0x20203030 false true 'Hello' \"THis is a string\\\" \"";

        let mut context = ParseContext::new(source);
        let lits = literal
            .wrap(eat_ignorable, eat_ignorable)
            .repeated()
            .parse(&mut context)
            .unwrap();
        println!("Remaining: {:?}", context.remaining());

        for lit in lits {
            println!("{lit:?}");
            println!("Remaining: {:?}", context.remaining());

            let root = SyntaxNode::new_root(lit.into_node().unwrap());
            println!("SyntaxNode: {root:#?}");
            let literal = Literal::cast(root).unwrap();
            println!("literal: {:?}", literal.to_string());
        }

        for error in context.errors().iter() {
            println!("Error: {error}");
        }

        /*         for segment in literal. {
            println!("Segment Name: {}", segment.segment().unwrap());
        } */
    }

    #[test]
    fn test_parser() {
        /*         use guano_syntax::consts::punctuation::ALL;

        println!("Unsorted: {ALL:#?}");

        let mut sorted = ALL.to_owned();
        sorted.sort_by(|x, y| y.cmp(&x));

        println!("Sorted: {sorted:#?}");

        println!("{}", PUNCT_REGEX.deref()); */

        let source = "\t\t\n\n\n\r\r\r\r\r  hfsoi isafhiuhasoeiuoaieshfoiuaweh iden this not sefojseijfoisjefoij kasfjhbfjhb      ";

        let mut parser = ParseContext::new(source);

        let results = ParseContext::raw_identifier
            .prefixed(ParseContext::eat_whitespace)
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
