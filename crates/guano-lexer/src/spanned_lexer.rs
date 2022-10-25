use std::vec::IntoIter;

use logos::{Lexer, Logos, Span as ByteSpan};

use crate::token::Token;

#[derive(Debug, Clone)]
pub struct NewSpannedLexer<Token>(IntoIter<(Token, Option<ByteSpan>)>);

impl NewSpannedLexer<Token> {
    pub fn new(lexer: Lexer<'_, Token>) -> Self {
        Self(
            lexer
                .spanned()
                .filter(|(t, _)| !matches!(t, Token::CommMulti(_) | Token::CommSingle(_)))
                .map(|(t, r)| (t, Some(r)))
                .collect::<Vec<_>>()
                .into_iter(),
        )
    }
}

impl<'source, Token: Logos<'source>> Iterator for NewSpannedLexer<Token> {
    type Item = (Token, Option<ByteSpan>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}