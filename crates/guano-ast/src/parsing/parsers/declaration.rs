use guano_syntax::Child;

use crate::parsing::{combinators::alternation, error::Res, ParseContext, Parser};

pub mod class;
pub mod function;
pub mod import;
pub mod module;
pub mod prototype;
pub mod variable;

pub fn decl<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    alternation((
        variable::var,
        function::func,
        class::class,
        import::import,
        prototype::proto,
    ))
    .parse(context)
}
