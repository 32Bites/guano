use guano_common::konst::format::formatcp;
use guano_syntax::Node;

use crate::ast::parsing::{combinators::alternation, error::Res, ParseContext, Parser};

pub mod float;
pub mod int;

const DECIMAL_REGEX: &'static str = r"(?:[0-9][_0-9]*)";
const BINARY_REGEX: &'static str = r"(?:0[bB][01][_01]*)";
const HEX_REGEX: &'static str = r"(?:0[xX][0-9A-Fa-f][_0-9A-Fa-f]*)";

const INTEGER_REGEX: &'static str = formatcp!("^(?:{BINARY_REGEX}|{HEX_REGEX}|{DECIMAL_REGEX})");
const FLOAT_REGEX: &'static str = formatcp!(r"^{DECIMAL_REGEX}\.{DECIMAL_REGEX}");

pub fn number_lazy<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    alternation((float::float_lazy, int::integer_lazy)).parse(context)
}
