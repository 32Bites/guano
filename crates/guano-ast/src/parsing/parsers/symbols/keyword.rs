use guano_syntax::{
    consts::{self, Keyword},
    leaf, Node,
};

use crate::parsing::{
    combinators::Combinators,
    error::{Error, ErrorKind, Res},
    ParseContext, Parser,
};

use super::iden::raw_iden;

pub fn keyword<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let (raw_iden, span) = raw_iden.spanned().peek().parse(context)?;

    for keyword in consts::Keyword::ALL {
        if keyword.as_str() == raw_iden {
            *context.position_mut() += span.len();
            return Ok(leaf(keyword.syntax_kind(), raw_iden));
        }
    }

    let kind = ErrorKind::String("Expected keyword".into());
    Err(Error::spanned(context.span(), kind))
}

impl<'source> Parser<'source> for Keyword {
    type Output = Node;
    type Error = Error<'source>;

    fn parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        let (keyword, span) = keyword.spanned().peek().parse(context).map_err(|mut e| {
            e.kind = ErrorKind::String(format!("Expected {:?}", self.as_str()).into());
            e
        })?;

        if keyword.kind().0 == self.syntax_kind() as u16 {
            *context.position_mut() += span.len();
            Ok(keyword)
        } else {
            let kind = ErrorKind::String(format!("Expected {:?}", self.as_str()).into());
            Err(Error::spanned(context.span(), kind))
        }
    }
}
