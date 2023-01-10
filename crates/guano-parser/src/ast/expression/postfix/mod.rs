use crate::ast::prelude::*;

mod call;
mod cast;
mod field;
mod index;

pub use call::*;
pub use cast::*;
pub use field::*;
pub use index::*;

pub fn parse_postfix_expression(original: Span) -> Res<Expr> {
    let (mut input, mut expr) = parse_primary_expression(original.clone())?;

    loop {
        let (new_input, result) = opt(alt((
            Field::parse(original.clone(), &expr),
            Index::parse(original.clone(), &expr),
            Call::parse(original.clone(), &expr),
        )))(input)?;

        input = new_input;

        match result {
            Some(e) => expr = e,
            None => break,
        }
    }

    Ok((input, expr))
}

pub fn parse_cast_expression(original: Span) -> Res<Expr> {
    let (mut input, mut expr) = parse_postfix_expression(original.clone())?;

    loop {
        let (new_input, result) = opt(Cast::parse(original.clone(), &expr))(input)?;
        input = new_input;

        match result {
            Some(e) => expr = e,
            None => break,
        }
    }

    Ok((input, expr))
}
