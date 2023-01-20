#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::iter::FilterMap;

use guano_common::rowan::{self, ast::AstNode, GreenNode, GreenToken, NodeOrToken};

/// Constants
pub mod consts;
/// Node data structures
pub mod nodes;
/// Token data structures
pub mod tokens;

pub type SyntaxNode = rowan::SyntaxNode<Lang>;
pub type SyntaxToken = rowan::SyntaxToken<Lang>;
pub type SyntaxElement = rowan::SyntaxElement<Lang>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<Lang>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<Lang>;
pub type PreorderWithTokens = rowan::api::PreorderWithTokens<Lang>;
pub type Node = NodeOrToken<GreenNode, GreenToken>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// Language type for use in [rowan]
pub enum Lang {}

impl rowan::Language for Lang {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        use ::guano_common::num::traits::FromPrimitive;
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        use ::guano_common::num::traits::ToPrimitive;
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}

/// Helper function for creating a leaf node
pub fn leaf(kind: crate::SyntaxKind, text: &str) -> Node {
    NodeOrToken::Token(GreenToken::new(kind.into(), text))
}

/// Helper function for creating a node
pub fn node(kind: crate::SyntaxKind, children: Vec<Node>) -> Node {
    NodeOrToken::Node(GreenNode::new(kind.into(), children))
}

pub fn error(text: &str) -> Node {
    NodeOrToken::Token(GreenToken::new(SyntaxKind::ERROR.into(), text))
}

/// Like [AstNode](rowan::ast::AstNode), but wraps tokens rather than interior nodes.
pub trait AstToken {
    fn can_cast(token: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxToken;

    #[inline]
    fn text(&self) -> &str {
        self.syntax().text()
    }
}

type CommentIter = FilterMap<SyntaxElementChildren, fn(SyntaxElement) -> Option<tokens::Comment>>;
type WsIter = FilterMap<SyntaxElementChildren, fn(SyntaxElement) -> Option<tokens::Whitespace>>;

#[inline]
fn comment_filter(n: SyntaxElement) -> Option<tokens::Comment> {
    match n {
        NodeOrToken::Node(_) => None,
        NodeOrToken::Token(token) => tokens::Comment::cast(token),
    }
}

#[inline]
fn ws_filter(n: SyntaxElement) -> Option<tokens::Whitespace> {
    match n {
        NodeOrToken::Node(_) => None,
        NodeOrToken::Token(token) => tokens::Whitespace::cast(token),
    }
}

pub trait NodeExt: AstNode<Language = Lang> {
    #[inline]
    fn comments(&self) -> CommentIter {
        self.syntax()
            .children_with_tokens()
            .filter_map(comment_filter)
    }

    #[inline]
    fn whitespace(&self) -> WsIter {
        self.syntax().children_with_tokens().filter_map(ws_filter)
    }
}

impl<T: AstNode<Language = Lang>> NodeExt for T {}

include!(concat!(env!("OUT_DIR"), "/generated_lib.rs"));
