use guano_syntax::Node;

use crate::parsing::{combinators::alternation, error::Res, ParseContext, Parser};

pub mod var;

pub fn decl<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    alternation((var::var,))
        // .map(|n| node(SyntaxKind::DECL, vec![n]))
        .parse(context)
}
