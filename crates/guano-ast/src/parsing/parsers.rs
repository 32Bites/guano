use guano_syntax::{node, Child, SyntaxKind};

use self::{declaration::module::module_items, ignorable::IgnorableParser};

use super::{error::Res, ParseContext, Parser};

/// A declaration is anything that brings a new
/// named item into scope.
pub mod declaration;
/// Expressions represent value.
pub mod expression;
/// Comments and Whitespace.
pub mod ignorable;
/// Implementation details for types
/// and prototypes
pub mod implementation;
/// Syntax punctuation
pub mod punctuation;
/// Named items or keywords
pub mod symbols;

pub fn source_file<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (l_ws, items, r_ws) = module_items.padded().parse(context)?;

    let mut children = l_ws;
    children.extend(items);
    children.extend(r_ws);

    Ok(node(SyntaxKind::SOURCE_FILE, children))
}
