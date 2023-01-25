use guano_syntax::{leaf, Node, SyntaxKind};

use crate::ast::parsing::{
    combinators::{regex, Combinators},
    error::{Error, ErrorKind, Res},
    ParseContext, Parser,
};

#[inline]
pub fn raw_iden<'source>(context: &mut ParseContext<'source>) -> Res<'source> {
    regex(r"^[_a-zA-Z][_0-9a-zA-Z]*").parse(context)
}

pub fn iden<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let (iden, span) = raw_iden.spanned().parse(context)?;
    let is_keyword = guano_syntax::consts::Keyword::ALL
        .into_iter()
        .map(|k| k.as_str())
        .any(|s| s == iden);

    if is_keyword {
        return Err(Error::spanned(
            span,
            ErrorKind::String("Found keyword, not identifier".into()),
        ));
    }

    Ok(leaf(SyntaxKind::IDEN, iden))
}
