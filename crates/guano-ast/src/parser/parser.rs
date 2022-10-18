use std::{
    iter::Filter,
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

    pub fn peek_token<const N: usize>(&mut self) -> [Option<Token>; N] {
        const INIT: Option<Token> = None;
        let mut tokens = [INIT; N];

        for i in 0..N {
            tokens[i] = self.lexer.peek().map(|(t, _)| t.clone());
        }

        tokens
    }

    pub fn peek_span<const N: usize>(&mut self) -> [Option<Span>; N] {
        const INIT: Option<Span> = None;
        let mut spans = [INIT; N];

        for i in 0..N {
            spans[i] = self.lexer.peek().map(|(_, s)| s.clone());
        }

        spans
    }

    pub fn peek<const N: usize>(&mut self) -> [Option<(Token, Span)>; N] {
        const INIT: Option<(Token, Span)> = None;
        let mut tokens_spanned = [INIT; N]; // Workaround

        for i in 0..N {
            tokens_spanned[i] = self.lexer.peek().cloned();
        }

        tokens_spanned
    }

    pub fn reset_peek(&mut self) {
        self.lexer.reset_peek()
    }

    pub fn read_token<const N: usize>(&mut self) -> [Option<Token>; N] {
        const INIT: Option<Token> = None;
        let mut tokens = [INIT; N];

        for i in 0..N {
            tokens[i] = self.lexer.next().map(|(t, _)| t);
        }

        tokens
    }

    pub fn read_span<const N: usize>(&mut self) -> [Option<Span>; N] {
        const INIT: Option<Span> = None;
        let mut spans = [INIT; N];

        for i in 0..N {
            spans[i] = self.lexer.next().map(|(_, s)| s);
        }

        spans
    }

    pub fn read<const N: usize>(&mut self) -> [Option<(Token, Span)>; N] {
        const INIT: Option<(Token, Span)> = None;
        let mut tokens_spanned = [INIT; N];

        for i in 0..N {
            tokens_spanned[i] = self.lexer.next();
        }

        tokens_spanned
    }
}

pub trait Parse<I: Iterator<Item = (Token, Span)>, E: std::error::Error, T = Self> {
    fn parse(parser: &mut Parser<I>) -> Result<T, Option<E>>;
}
