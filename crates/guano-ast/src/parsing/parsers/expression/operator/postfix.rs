use guano_common::num::traits::FromPrimitive;
use guano_syntax::{
    consts::{Keyword, Punctuation},
    node, Child, SyntaxKind,
};

use crate::parsing::{
    combinators::{alternation, tuple, Combinators},
    error::Res,
    parsers::{
        expression::{
            expr,
            pratt::{Postfix, Power},
            primary::list::list_expr_items,
        },
        ignorable::{eat_ignorable, IgnorableParser},
        symbols::{identifier::iden, ty::ty},
    },
    ParseContext, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostfixKind {
    Field,
    Index,
    Call,
    Cast,
    Is,
}

impl PostfixKind {
    pub fn parse_expr<'source>(
        &self,
        lhs: &mut Child,
        context: &mut ParseContext<'source>,
    ) -> Res<'source, ()> {
        match self {
            PostfixKind::Field => Self::field(lhs, context),
            PostfixKind::Index => Self::index(lhs, context),
            PostfixKind::Call => Self::call(lhs, context),
            PostfixKind::Cast => Self::typed(true, lhs, context),
            PostfixKind::Is => Self::typed(false, lhs, context),
        }
    }

    fn call<'source>(lhs: &mut Child, context: &mut ParseContext<'source>) -> Res<'source, ()> {
        let (ws, left_paren, params, right_paren) = tuple((
            eat_ignorable,
            Punctuation::LEFT_PAREN,
            list_expr_items,
            Punctuation::RIGHT_PAREN.expected(),
        ))
        .parse(context)?;

        take_mut::take(lhs, |lhs| {
            let mut children = vec![lhs];
            children.extend(ws);
            children.push(left_paren);
            children.extend(params);
            children.push(right_paren);

            let call = node(SyntaxKind::CALL_EXPR, children);
            // let expr = node(SyntaxKind::EXPR, vec![call]);

            call
        });

        Ok(())
    }

    fn typed<'source>(
        is_cast: bool,
        lhs: &mut Child,
        context: &mut ParseContext<'source>,
    ) -> Res<'source, ()> {
        let ((r_ws, kw, l_ws), ty) = tuple((
            alternation((Keyword::AS, Keyword::IS)).padded(),
            ty.expected(),
        ))
        .parse(context)?;

        take_mut::take(lhs, |lhs| {
            let mut children = vec![lhs];
            children.extend(r_ws);
            children.push(kw);
            children.extend(l_ws);
            children.push(ty);

            let kind = if is_cast {
                SyntaxKind::CAST_EXPR
            } else {
                SyntaxKind::IS_EXPR
            };

            let typed = node(kind, children);
            // let expr = node(SyntaxKind::EXPR, vec![typed]);

            typed
        });

        Ok(())
    }

    fn index<'source>(lhs: &mut Child, context: &mut ParseContext<'source>) -> Res<'source, ()> {
        let (first_ws, left_brack, (left_ws, expr, right_ws), right_brack) = tuple((
            eat_ignorable,
            Punctuation::LEFT_BRACK,
            expr.expected().padded(),
            Punctuation::RIGHT_BRACK.expected(),
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
            // let expr = node(SyntaxKind::EXPR, vec![index]);

            index
        });

        Ok(())
    }

    fn field<'source>(lhs: &mut Child, context: &mut ParseContext<'source>) -> Res<'source, ()> {
        let (left_ws, dot, right_ws, rhs) = tuple((
            eat_ignorable,
            Punctuation::DOT,
            eat_ignorable,
            iden.expected(),
        ))
        .parse(context)?;

        take_mut::take(lhs, |lhs| {
            let mut children = vec![lhs];
            children.extend(left_ws);
            children.push(dot);
            children.extend(right_ws);
            children.push(rhs);

            let field = node(SyntaxKind::FIELD_EXPR, children);
            // let expr = node(SyntaxKind::EXPR, vec![field]);

            field
        });

        Ok(())
    }
}

impl Postfix for PostfixKind {
    fn power(&self) -> Power {
        match self {
            PostfixKind::Field | PostfixKind::Index | PostfixKind::Call => 12,
            PostfixKind::Cast | PostfixKind::Is => 10,
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
        Keyword::IS,
    ))
    .prefixed(eat_ignorable)
    .parse(context)?;

    let kind = SyntaxKind::from_u16(node.kind().0).unwrap();

    let kind = match kind {
        SyntaxKind::DOT => PostfixKind::Field,
        SyntaxKind::LEFT_BRACK => PostfixKind::Index,
        SyntaxKind::LEFT_PAREN => PostfixKind::Call,
        SyntaxKind::KW_AS => PostfixKind::Cast,
        SyntaxKind::KW_IS => PostfixKind::Is,
        _ => unreachable!(),
    };

    Ok(kind)
}
