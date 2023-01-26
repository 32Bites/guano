use guano_syntax::{consts, leaf, Child};

use crate::parsing::{
    combinators::{tag, Combinators},
    error::{Error, ErrorKind, Res},
    ParseContext, Parser,
};

pub fn punctuation<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    for mark in consts::Punctuation::ALL {
        match tag(mark.as_str()).optional().parse(context)? {
            Some(text) => return Ok(leaf(mark.syntax_kind(), text)),
            None => continue,
        }
    }

    let kind = ErrorKind::String("Expected punctuation".into());

    Err(Error::spanned(context.span(), kind))
}

impl<'source> Parser<'source> for consts::Punctuation {
    type Output = Child;
    type Error = Error<'source>;

    fn parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        let (punct, span) = punctuation.spanned().peek().parse(context)?;
        if punct.kind().0 == self.syntax_kind() as u16 {
            *context.position_mut() += span.len();

            Ok(punct)
        } else {
            let kind =
                ErrorKind::String(format!("Expected punctuation {:?}", self.as_str()).into());
            Err(Error::spanned(context.span(), kind))
        }
    }
}

pub trait PunctuationExt<'source> {
    fn punctuation(&mut self) -> Res<'source, Child>;
}

impl<'source> PunctuationExt<'source> for ParseContext<'source> {
    #[inline]
    fn punctuation(&mut self) -> Res<'source, Child> {
        punctuation(self)
    }
}
