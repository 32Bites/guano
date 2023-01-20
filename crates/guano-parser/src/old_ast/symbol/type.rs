use crate::ast::prelude::*;
use guano_common::rowan::{GreenNode, NodeOrToken};
use guano_syntax::{
    parser::{
        punctuation::{left_brack, ques, right_brack},
        wrap, Input, Res as Result,
    },
    SyntaxKind,
};

pub fn r#type<'a>(input: Input<'a>) -> Result<'a> {
    nilable_type(input)
}

pub fn nilable_type<'a>(input: Input<'a>) -> Result<'a> {
    let (input, (mut base, marks)) = pair(base_type, many0(pad_l(ques)))(input)?;

    for (ignored, mark) in marks {
        let capacity = 2 + ignored.len();
        let mut children = Vec::with_capacity(capacity);
        children.push(base);
        children.extend(ignored);
        children.push(mark);

        let nilable = NodeOrToken::Node(GreenNode::new(SyntaxKind::NILABLE_TYPE.into(), children));
        base = NodeOrToken::Node(GreenNode::new(SyntaxKind::TYPE.into(), [nilable]));
    }

    Ok((input, base))
}

pub fn base_type<'a>(input: Input<'a>) -> Result<'a> {
    alt((wrap(path, SyntaxKind::TYPE), list_type))(input)
}

pub fn list_type<'a>(input: Input<'a>) -> Result<'a> {
    let (input, (l_brack, (l_ignored, ty, r_ignored), r_brack)) =
        tuple((left_brack, pad(r#type), right_brack))(input)?;

    let capacity = 3 + l_ignored.len() + r_ignored.len();
    let mut children = Vec::with_capacity(capacity);

    children.push(l_brack);
    children.extend(l_ignored);
    children.push(ty);
    children.extend(r_ignored);
    children.push(r_brack);

    let mut node = NodeOrToken::Node(GreenNode::new(SyntaxKind::LIST_TYPE.into(), children));
    node = NodeOrToken::Node(GreenNode::new(SyntaxKind::TYPE.into(), [node]));

    Ok((input, node))
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Type {
    kind: TypeKind,
    span: NodeSpan,
}

impl Type {
    pub fn kind(&self) -> &TypeKind {
        &self.kind
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Self> {
        let (input, (span, kind)) = consumed(TypeKind::parse)(input)?;

        let ty = Self {
            kind,
            span: span.into_node(),
        };

        Ok((input, ty))
    }
}

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a Type {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        self.kind.pretty(allocator)
    }
}

impl Node for Type {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Named(Path),
    List(Box<Type>),
}

impl Default for TypeKind {
    fn default() -> Self {
        Self::Named(Path::default())
    }
}

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a TypeKind {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        match self {
            TypeKind::Named(p) => p.pretty(allocator),
            TypeKind::List(ty) => allocator.text("[]").append(&**ty),
        }
    }
}

impl std::fmt::Display for TypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeKind::Named(n) => n.fmt(f),
            TypeKind::List(t) => write!(f, "[]{t}"),
        }
    }
}

impl TypeKind {
    fn parse(input: Span) -> Res<Self> {
        let named = map(Path::parse, Self::Named);
        let list = map(
            preceded(
                tuple((tag("["), ignorable, tag("]"))),
                preceded(ignorable, Type::parse),
            ),
            |t| TypeKind::List(Box::new(t)),
        );

        alt((named, list))(input)
    }
}
