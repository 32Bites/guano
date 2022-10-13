use std::{
    error::Error,
    iter::{Filter, Peekable},
};

use guano_lexer::{logos::Logos, Span, SpannedLexer, ToSpanned, Token};
use itertools::{Itertools, MultiPeek};

#[derive(Debug)]
pub enum ParseState {
    //Expression(ExpressionParser),
}

#[derive(Debug)]
pub struct Parser<I: Iterator<Item = (Token, Span)> = Box<dyn Iterator<Item = (Token, Span)>>> {
    // pub sources: HashMap<PathBuf, u8>,
    pub stack: Vec<ParseState>,
    pub lexer: MultiPeek<I>,
}

impl<I: Iterator<Item = (Token, Span)>> Parser<I> {
    pub fn new(iterator: I) -> Parser<Filter<I, fn(&(Token, Span)) -> bool>> {
        fn filter((t, _): &(Token, Span)) -> bool {
            !matches!(t, Token::CommMulti(_) | Token::CommSingle(_))
        }
        Parser {
            stack: vec![],
            lexer: iterator
                .filter::<fn(&(Token, Span)) -> bool>(filter)
                .multipeek(),
        }
    }

    pub fn from_source<'a>(
        source: &'a str,
    ) -> Parser<Filter<SpannedLexer<'a, Token>, fn(&(Token, Span)) -> bool>> {
        Parser::new(Token::lexer(source).to_spanned())
    }
}

pub trait Parse<T = Self> {
    fn parse(parser: &mut Parser<impl Iterator<Item = (Token, Span)>>) -> Option<T>;
}
