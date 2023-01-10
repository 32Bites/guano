use crate::ast::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Type {
    kind: TypeKind,
    #[cfg_attr(feature = "serde", serde(skip))]
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

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl Node for Type {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TypeKind {
    Named(Path),
    List(Box<Type>),
}

impl Default for TypeKind {
    fn default() -> Self {
        Self::Named(Path::default())
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
