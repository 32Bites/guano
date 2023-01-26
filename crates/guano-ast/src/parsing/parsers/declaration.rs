use guano_syntax::Child;

use crate::parsing::{combinators::alternation, error::Res, ParseContext, Parser};

pub mod function;
pub mod variable;

pub fn decl<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    alternation((variable::var, function::func))
        // .map(|n| node(SyntaxKind::DECL, vec![n]))
        .parse(context)
}
