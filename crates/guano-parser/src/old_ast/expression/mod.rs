use bigdecimal::FromPrimitive;
use guano_common::rowan::{ast::AstNode, GreenNode, NodeOrToken};
use guano_syntax::{
    parser::{Input, Res as Result},
    SyntaxKind,
};
use nom::{branch::alt, combinator::flat_map};

// use crate::ast::prelude::*;

use self::{operator::new::parse_unary, primary::primary_expr};

use super::{
    node::Node,
    prelude::pad_r,
    span::{NodeSpan, Span},
    Res,
};

pub mod binary;
pub mod block;
pub mod operator;
pub mod postfix;
pub mod primary;
pub mod unary;

pub fn expr<'a>(input: Input<'a>) -> Result<'a> {
    alt((block::block_expr, |input| pratt_expr(input, 0)))(input)
}

pub fn pratt_expr<'a>(input: Input<'a>, min_power: u32) -> Result<'a> {
    let unary = flat_map(pad_r(parse_unary), |unary_op| {
        move |input| {
            let (unary_op, ignored) = unary_op.clone();
            let ((), right_power) = operator::new::UnaryOperator::from_op(
                SyntaxKind::from_u16(unary_op.kind().0).unwrap(),
            )
            .power();

            let (input, value) = pratt_expr(input, right_power)?;

            let capacity = 2 + ignored.len();

            let mut children = Vec::with_capacity(capacity);
            children.push(unary_op);
            children.extend(ignored);
            children.push(value);

            let mut node =
                NodeOrToken::Node(GreenNode::new(SyntaxKind::UNARY_EXPR.into(), children));
            node = NodeOrToken::Node(GreenNode::new(SyntaxKind::EXPR.into(), [node]));

            Ok((input, node))
        }
    });

    #[allow(unused_mut)]
    let (mut input, mut lhs) = alt((unary, primary_expr))(input)?;

    /*     loop {
        input = input;
        lhs = lhs;
    } */

    Ok((input, lhs))
}

#[derive(Debug, Clone, Default)]
pub struct Expr {
    span: NodeSpan,
    kind: ExprKind,
}

impl Expr {
    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn kind(&self) -> &ExprKind {
        &self.kind
    }

    fn new(kind: ExprKind, span: NodeSpan) -> Expr {
        Expr { span, kind }
    }

    pub fn parse(input: Span) -> Res<Expr> {
        binary::Binary::parse(input)
    }
}

impl Expr {
    pub fn is_block(&self) -> bool {
        self.kind.is_block()
    }
}

/* impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
} */

impl Node for Expr {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a Expr {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        self.kind.pretty(allocator)
    }
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Literal(super::prelude::Lit),
    Path(super::prelude::Path),
    If(primary::If),
    While(primary::While),
    For(primary::For),
    Loop(primary::Loop),
    Block(super::block::Block),
    Unary(unary::Unary),
    Binary(binary::Binary),
    Cast(postfix::Cast),
    Field(postfix::Field),
    This(primary::This),
    Index(postfix::Index),
    Call(postfix::Call),
    List(primary::List),
    Group(primary::Group),
    Return(primary::Return),
    Continue,
    Break,
}

impl ExprKind {
    pub fn is_block(&self) -> bool {
        use ExprKind::{Block, For, If, Loop, While};
        matches!(self, If(_) | While(_) | For(_) | Loop(_) | Block(_))
    }
}

impl Default for ExprKind {
    fn default() -> Self {
        ExprKind::Literal(super::prelude::Lit::default())
    }
}

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a ExprKind {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        match self {
            ExprKind::Literal(l) => l.pretty(allocator),
            ExprKind::Path(p) => p.pretty(allocator),
            ExprKind::If(i) => i.pretty(allocator),
            ExprKind::While(w) => w.pretty(allocator),
            ExprKind::For(f) => f.pretty(allocator),
            ExprKind::Loop(l) => l.pretty(allocator),
            ExprKind::Block(b) => b.pretty(allocator),
            ExprKind::Unary(u) => u.pretty(allocator),
            ExprKind::Binary(b) => b.pretty(allocator),
            ExprKind::Cast(c) => c.pretty(allocator),
            ExprKind::Field(f) => f.pretty(allocator),
            ExprKind::This(t) => t.pretty(allocator),
            ExprKind::Index(i) => i.pretty(allocator),
            ExprKind::Call(c) => c.pretty(allocator),
            ExprKind::List(l) => l.pretty(allocator),
            ExprKind::Group(g) => g.pretty(allocator),
            ExprKind::Return(r) => r.pretty(allocator),
            ExprKind::Continue => allocator.text("continue"),
            ExprKind::Break => allocator.text("break"),
        }
    }
}

/* impl std::fmt::Display for ExprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprKind::Literal(l) => l.fmt(f),
            ExprKind::Path(p) => p.fmt(f),
            ExprKind::If(i) => i.fmt(f),
            ExprKind::While(w) => w.fmt(f),
            ExprKind::For(fo) => fo.fmt(f),
            ExprKind::Loop(l) => l.fmt(f),
            ExprKind::Block(b) => b.fmt(f),
            ExprKind::Unary(u) => u.fmt(f),
            ExprKind::Binary(b) => b.fmt(f),
            ExprKind::Cast(c) => c.fmt(f),
            ExprKind::Field(fi) => fi.fmt(f),
            ExprKind::Index(i) => i.fmt(f),
            ExprKind::Call(c) => c.fmt(f),
            ExprKind::List(l) => l.fmt(f),
            ExprKind::Group(g) => g.fmt(f),
            ExprKind::Return(r) => r.fmt(f),
            ExprKind::Continue => f.write_str("continue"),
            ExprKind::Break => f.write_str("break"),
            ExprKind::This(t) => t.fmt(f),
        }
    }
} */
