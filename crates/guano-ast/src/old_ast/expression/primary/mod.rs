use crate::ast::prelude::*;
use guano_syntax::{
    parser::{keyword::{kw_continue, kw_break}, wrap, Input, Res as Result},
    SyntaxKind,
};

mod r#for;
mod group;
mod r#if;
mod list;
mod literal;
mod r#loop;
mod r#return;
mod this;
mod r#while;

pub use group::*;
pub use list::*;
pub use literal::*;
pub use r#for::*;
pub use r#if::*;
pub use r#loop::*;
pub use r#return::*;
pub use r#while::*;
pub use this::*;

pub fn break_expr<'a>(input: Input<'a>) -> Result<'a> {
    wrap(
        wrap(
            kw_break(crate::ast::symbol::iden::parse_raw),
            SyntaxKind::BREAK_EXPR,
        ),
        SyntaxKind::EXPR,
    )(input)
}

pub fn continue_expr<'a>(input: Input<'a>) -> Result<'a> {
    wrap(
        wrap(
            kw_continue(crate::ast::symbol::iden::parse_raw),
            SyntaxKind::CONTINUE_EXPR,
        ),
        SyntaxKind::EXPR,
    )(input)
}

pub fn path_expr<'a>(input: Input<'a>) -> Result<'a> {
    wrap(path, SyntaxKind::EXPR)(input)
}

pub fn primary_expr<'a>(input: Input<'a>) -> Result<'a> {
    alt((
        return_expr,
        break_expr,
        continue_expr,
        path_expr,
        literal_expr,
        group_expr,
        list_expr,
/*         for_expr,
        if_expr,
        loop_expr,
        while_expr, */
        this_expr,
    ))(input)
}

pub fn parse_primary_expression(input: Span) -> Res<Expr> {
    alt((
        parse_block_expression,
        If::parse,
        While::parse,
        Loop::parse,
        For::parse,
        Lit::parse,
        parse_path_expression,
        parse_continue_or_break_expression,
        Return::parse,
        This::parse,
        List::parse,
        Group::parse,
    ))(input)
}

pub fn parse_continue_or_break_expression(input: Span) -> Res<Expr> {
    map(
        consumed(alt((
            value(ExprKind::Continue, Keyword::Continue),
            value(ExprKind::Break, Keyword::Break),
        ))),
        |(span, kind)| Expr::new(kind, span.into_node()),
    )(input)
}

pub fn parse_path_expression(input: Span) -> Res<Expr> {
    map(consumed(Path::parse), |(s, p)| {
        Expr::new(ExprKind::Path(p), s.into_node())
    })(input)
}

pub fn parse_block_expression(input: Span) -> Res<Expr> {
    let (input, blk) = Block::parse(input)?;
    let span = blk.span().clone();

    let kind = ExprKind::Block(blk);
    let expr = Expr::new(kind, span);

    Ok((input, expr))
}
