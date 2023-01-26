use guano_syntax::Child;

use crate::parsing::{
    combinators::alternation, error::Res, parsers::symbols::path, ParseContext, Parser,
};

use self::keyword::{break_expr, continue_expr, return_expr};

use super::block::block;

pub mod group;
pub mod keyword;
pub mod list;
pub mod literal;

pub fn primary<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    alternation((
        block,
        group::group_expr,
        list::list_expr,
        literal::literal,
        path::path,
        return_expr,
        continue_expr,
        break_expr,
    ))
    .parse(context)
}
