use std::{ops::Range, vec};

use guano_parser::ast::{
    expression::{
        binary::Binary,
        operator,
        postfix::{Call, Cast, Field, Index},
        primary::{For, Group, If, List, Lit, LitKind, Loop, Return, This, While},
        unary::Unary,
        Expr, ExprKind,
    },
    span::{SpanExt, Spanned},
    symbol::{
        iden::Iden,
        path::Path,
        ty::{Type, TypeKind},
    },
};

use crate::tree::{Element, Tree};

impl From<&Iden> for Tree<Range<usize>> {
    fn from(iden: &Iden) -> Self {
        Tree::Leaf(
            Element::new(
                '🏷',
                (**iden.span().fragment()).into(),
                Some("Identifier".into()),
            ),
            iden.span().to_range(),
        )
    }
}

impl From<&Path> for Tree<Range<usize>> {
    fn from(path: &Path) -> Self {
        Tree::List {
            name: Element::new('🗺', "Path".into(), Some("Path".into())),
            items: path.segments().iter().map(|s| s.into()).collect(),
            extra: path.span().to_range(),
        }
    }
}

impl From<&Type> for Tree<Range<usize>> {
    fn from(ty: &Type) -> Self {
        match ty.kind() {
            TypeKind::Named(p) => p.into(),
            TypeKind::List(l) => Tree::List {
                name: Element::new('[', "List".into(), Some("List Type".into())),
                items: vec![l.as_ref().into()],
                extra: ty.span().to_range(),
            },
        }
    }
}

impl From<&Expr> for Tree<Range<usize>> {
    fn from(expr: &Expr) -> Self {
        match expr.kind() {
            ExprKind::Literal(l) => l.into(),
            ExprKind::Path(p) => p.into(),
            ExprKind::If(i) => i.into(),
            ExprKind::While(w) => w.into(),
            ExprKind::For(f) => f.into(),
            ExprKind::Loop(l) => l.into(),
            ExprKind::Block(b) => todo!(), // b.into(),
            ExprKind::Unary(u) => u.into(),
            ExprKind::Binary(b) => b.into(),
            ExprKind::Cast(c) => c.into(),
            ExprKind::Field(f) => f.into(),
            ExprKind::This(t) => t.into(),
            ExprKind::Index(i) => i.into(),
            ExprKind::Call(c) => c.into(),
            ExprKind::List(l) => l.into(),
            ExprKind::Group(g) => g.into(),
            ExprKind::Return(r) => r.into(),
            ExprKind::Continue => Tree::Leaf("Continue".into(), expr.span().to_range()),
            ExprKind::Break => Tree::Leaf("Break".into(), expr.span().to_range()),
        }
    }
}

impl From<&Unary> for Tree<Range<usize>> {
    fn from(expr: &Unary) -> Self {
        Tree::List {
            name: expr.operator().name().into(),
            items: vec![expr.expr().into()],
            extra: expr.span().to_range(),
        }
    }
}

impl From<Spanned<operator::Binary>> for Tree<Range<usize>> {
    fn from(s: Spanned<operator::Binary>) -> Self {
        Tree::Leaf(s.value().as_str().into(), s.span().to_range())
    }
}

impl From<&Binary> for Tree<Range<usize>> {
    fn from(expr: &Binary) -> Self {
        Tree::Struct {
            name: Element::new(
                '±',
                expr.operator().name().into(),
                Some("Binary Expression".into()),
            ),
            items: vec![
                ("lhs".into(), expr.lhs().into()),
                ("rhs".into(), expr.rhs().into()),
            ],
            extra: expr.span().to_range(),
        }
    }
}

impl From<&Lit> for Tree<Range<usize>> {
    fn from(expr: &Lit) -> Self {
        let element = match expr.kind() {
            LitKind::Bool(b) => Element::new('✅', b.to_string(), Some("Boolean Literal".into())),
            LitKind::Integer(i) => {
                Element::new('🔢', i.to_string(), Some("Integer Literal".into()))
            }
            LitKind::Float(f) => Element::new('½', f.to_string(), Some("Float Literal".into())),
            LitKind::String(s) => {
                Element::new('📃', format!("{s:?}"), Some("String Literal".into()))
            }
            LitKind::Char(c) => {
                Element::new('🗛', format!("{c:?}"), Some("Character Literal".into()))
            }
            LitKind::Nil => return Tree::Leaf("Nil".into(), expr.span().to_range()),
        };

        Tree::Leaf(element, expr.span().to_range())
    }
}

impl From<&Group> for Tree<Range<usize>> {
    fn from(expr: &Group) -> Self {
        expr.expr().into()
    }
}

impl From<&List> for Tree<Range<usize>> {
    fn from(expr: &List) -> Self {
        Tree::List {
            name: Element::new(']', "List".into(), Some("List expression".into())),
            items: expr.expressions().iter().map(|e| e.into()).collect(),
            extra: expr.span().to_range(),
        }
    }
}

impl From<&Return> for Tree<Range<usize>> {
    fn from(expr: &Return) -> Self {
        match expr.expr() {
            Some(inner) => Tree::List {
                name: Element::new('🚩', "Return".into(), Some("Return expression".into())),
                items: vec![inner.into()],
                extra: expr.span().to_range(),
            },
            None => Tree::Leaf(
                Element::new('🚩', "Return".into(), Some("Return expression".into())),
                expr.span().to_range(),
            ),
        }
    }
}

impl From<&This> for Tree<Range<usize>> {
    fn from(expr: &This) -> Self {
        Tree::Leaf(
            Element::new(
                '＠',
                format!("@{}", expr.iden()),
                Some("This expression".into()),
            ),
            expr.span().to_range(),
        )
    }
}

impl From<&Call> for Tree<Range<usize>> {
    fn from(expr: &Call) -> Self {
        Tree::Struct {
            name: Element::new('📞', "Call".into(), Some("Call expression".into())),
            items: vec![
                ("expression".into(), expr.expr().into()),
                (
                    "parameters".into(),
                    Tree::List {
                        name: Element::new('⎆', "Parameters".into(), None),
                        items: expr.parameters().iter().map(|p| p.into()).collect(),
                        extra: expr.span().to_range(),
                    },
                ),
            ],
            extra: expr.span().to_range(),
        }
    }
}

impl From<&Cast> for Tree<Range<usize>> {
    fn from(expr: &Cast) -> Self {
        Tree::Struct {
            name: Element::new('🔀', "Cast".into(), Some("Cast expression".into())),
            items: vec![
                ("expression".into(), expr.expr().into()),
                ("type".into(), expr.ty().into()),
            ],
            extra: expr.span().to_range(),
        }
    }
}

impl From<&Field> for Tree<Range<usize>> {
    fn from(expr: &Field) -> Self {
        Tree::Struct {
            name: Element::new('.', "Field".into(), Some("Field expression".into())),
            items: vec![
                ("expression".into(), expr.expr().into()),
                ("identifier".into(), expr.iden().into()),
            ],
            extra: expr.span().to_range(),
        }
    }
}

impl From<&Index> for Tree<Range<usize>> {
    fn from(expr: &Index) -> Self {
        Tree::Struct {
            name: Element::new('☜', "Index".into(), Some("Index expression".into())),
            items: vec![
                ("expression".into(), expr.expr().into()),
                ("index".into(), expr.index().into()),
            ],
            extra: expr.span().to_range(),
        }
    }
}

impl From<&If> for Tree<Range<usize>> {
    fn from(expr: &If) -> Self {
        todo!()
    }
}

impl From<&For> for Tree<Range<usize>> {
    fn from(expr: &For) -> Self {
        todo!()
    }
}

impl From<&While> for Tree<Range<usize>> {
    fn from(expr: &While) -> Self {
        todo!()
    }
}

impl From<&Loop> for Tree<Range<usize>> {
    fn from(expr: &Loop) -> Self {
        todo!()
    }
}
