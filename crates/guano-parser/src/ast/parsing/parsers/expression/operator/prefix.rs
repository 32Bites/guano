use guano_common::{rowan::ast::AstNode, num::traits::FromPrimitive};
use guano_syntax::{nodes::UnaryOp, Node, SyntaxKind, node};

use crate::ast::parsing::{
    error::{Res, ErrorKind, Error},
    parsers::{expression::pratt::{Power, Prefix}, punctuation::punctuation},
    ParseContext, combinators::Combinators, Parser,
};

pub fn unary_op<'source>(context: &mut ParseContext<'source>) -> Res<'source, (Node, UnaryKind)> {
    let (mark, span) = punctuation.spanned().peek().parse(context)?;
    let kind = SyntaxKind::from_u16(mark.kind().0).unwrap();

    if let Some(kind) = UnaryKind::from_syntax(kind) {
        context.advance_byte(u32::from(span.len()) as usize)?;

        let node = node(SyntaxKind::UNARY_OP, vec![mark]);

        Ok((node, kind))
    } else {
        let kind = ErrorKind::String("Invalid unary operator".into());

        Err(Error::spanned(span, kind))
    }
}

impl Prefix for UnaryOp {
    #[inline]
    fn power(&self) -> Power {
        self.kind().power()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryKind {
    Negate,
    Not,
}

impl UnaryKind {
    pub fn from_syntax(kind: SyntaxKind) -> Option<Self> {
        match kind {
            SyntaxKind::MINUS => Some(Self::Negate),
            SyntaxKind::BANG => Some(Self::Not),
            _ => None
        }
    }
}

impl Prefix for UnaryKind {
    #[inline]
    fn power(&self) -> Power {
        11.into()
    }
}

pub trait UnaryExt {
    fn kind(&self) -> UnaryKind;
}

impl UnaryExt for UnaryOp {
    fn kind(&self) -> UnaryKind {
        match self.syntax().first_token().unwrap().kind() {
            SyntaxKind::MINUS => UnaryKind::Negate,
            SyntaxKind::BANG => UnaryKind::Not,
            _ => panic!("Invalid unary operator"),
        }
    }
}
