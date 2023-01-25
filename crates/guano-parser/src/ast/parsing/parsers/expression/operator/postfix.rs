use guano_common::num::traits::FromPrimitive;
use guano_syntax::{
    consts::{Keyword, Punctuation},
    node, Node, SyntaxKind,
};

use crate::ast::parsing::{
    combinators::{alternation, tuple, Combinators},
    error::Res,
    parsers::{
        expression::{
            expr,
            pratt::{Postfix, Power},
        },
        ignorable::{eat_ignorable, IgnorableParser},
        symbols::iden::iden,
    },
    ParseContext, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostfixKind {
    Field,
    Index,
    Call,
    Cast,
}

impl PostfixKind {
    pub fn parse_expr<'source>(
        &self,
        lhs: &mut Node,
        context: &mut ParseContext<'source>,
    ) -> Res<'source, ()> {
        match self {
            PostfixKind::Field => Self::field(lhs, context),
            PostfixKind::Index => Self::index(lhs, context),
            PostfixKind::Call => todo!(),
            PostfixKind::Cast => todo!(),
        }
    }

    fn index<'source>(lhs: &mut Node, context: &mut ParseContext<'source>) -> Res<'source, ()> {
        let (first_ws, left_brack, (left_ws, expr, right_ws), right_brack) = tuple((
            eat_ignorable,
            Punctuation::LEFT_BRACK,
            expr.padded(),
            Punctuation::RIGHT_BRACK,
        ))
        .parse(context)?;

        take_mut::take(lhs, |lhs| {
            let mut children = vec![lhs];
            children.extend(first_ws);
            children.push(left_brack);
            children.extend(left_ws);
            children.push(expr);
            children.extend(right_ws);
            children.push(right_brack);

            let index = node(SyntaxKind::INDEX_EXPR, children);
            let expr = node(SyntaxKind::EXPR, vec![index]);

            expr
        });

        Ok(())
    }

    fn field<'source>(lhs: &mut Node, context: &mut ParseContext<'source>) -> Res<'source, ()> {
        let (left_ws, dot, right_ws, rhs) =
            tuple((eat_ignorable, Punctuation::DOT, eat_ignorable, iden)).parse(context)?;

        take_mut::take(lhs, |lhs| {
            let mut children = vec![lhs];
            children.extend(left_ws);
            children.push(dot);
            children.extend(right_ws);
            children.push(rhs);

            let field = node(SyntaxKind::FIELD_EXPR, children);
            let expr = node(SyntaxKind::EXPR, vec![field]);

            expr
        });

        Ok(())
    }
}

impl Postfix for PostfixKind {
    fn power(&self) -> Power {
        match self {
            PostfixKind::Field | PostfixKind::Index | PostfixKind::Call => 12,
            PostfixKind::Cast => 10,
        }
        .into()
    }
}

pub fn postfix_operator<'source>(context: &mut ParseContext<'source>) -> Res<'source, PostfixKind> {
    let node = alternation((
        Punctuation::DOT,
        Punctuation::LEFT_BRACK,
        Punctuation::LEFT_PAREN,
        Keyword::AS,
    ))
    .prefixed(eat_ignorable)
    .parse(context)?;

    let kind = SyntaxKind::from_u16(node.kind().0).unwrap();

    let kind = match kind {
        SyntaxKind::DOT => PostfixKind::Field,
        SyntaxKind::LEFT_BRACK => PostfixKind::Index,
        SyntaxKind::LEFT_PAREN => PostfixKind::Call,
        SyntaxKind::KW_AS => PostfixKind::Cast,
        _ => unreachable!(),
    };

    Ok(kind)
}
