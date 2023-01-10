use crate::ast::prelude::*;

pub mod binary;
pub mod operator;
pub mod postfix;
pub mod primary;
pub mod unary;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Expr {
    #[cfg_attr(feature = "serde", serde(skip))]
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
        Binary::parse(input)
    }
}

impl Expr {
    pub fn is_block(&self) -> bool {
        self.kind.is_block()
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl Node for Expr {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ExprKind {
    Literal(Lit),
    Path(Path),
    If(If),
    While(While),
    For(For),
    Loop(Loop),
    Block(Block),
    Unary(Unary),
    Binary(Binary),
    Cast(Cast),
    Field(Field),
    This(This),
    Index(Index),
    Call(Call),
    List(List),
    Group(Group),
    Return(Return),
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
        ExprKind::Literal(Lit::default())
    }
}

impl std::fmt::Display for ExprKind {
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
}
