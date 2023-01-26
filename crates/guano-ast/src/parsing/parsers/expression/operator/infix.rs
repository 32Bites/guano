use guano_common::{num::traits::FromPrimitive, rowan::ast::AstNode};
use guano_syntax::{node, nodes::BinaryOp, Node, SyntaxKind};

use crate::parsing::{
    combinators::Combinators,
    error::{Error, ErrorKind, Res},
    parsers::{
        expression::pratt::{Associativity, Infix, Power},
        ignorable::eat_ignorable,
        punctuation::punctuation,
    },
    ParseContext, Parser,
};

pub fn binary_op<'source>(context: &mut ParseContext<'source>) -> Res<'source, (Node, BinaryKind)> {
    let (mark, span) = punctuation
        .prefixed(eat_ignorable)
        .spanned()
        .peek()
        .parse(context)?;
    let kind = SyntaxKind::from_u16(mark.kind().0).unwrap();
    if let Some(kind) = BinaryKind::from_syntax(kind) {
        context.advance_byte(u32::from(span.len()) as usize)?;
        let node = node(SyntaxKind::BINARY_OP, vec![mark]);

        Ok((node, kind))
    } else {
        let kind = ErrorKind::String("Invalid binary operator".into());

        Err(Error::spanned(span, kind))
    }
}

impl Infix for BinaryOp {
    #[inline]
    fn associativity(&self) -> Associativity {
        self.kind().associativity()
    }

    #[inline]
    fn power(&self) -> Power {
        self.kind().power()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryKind {
    Factor(Factor),
    Term(Term),
    Bitwise(Bitwise),
    Comparison(Comparison),
    Logical(Logical),
    Assignment(Assignment),
}

impl Infix for BinaryKind {
    #[inline]
    fn associativity(&self) -> Associativity {
        use Associativity::*;
        match self {
            BinaryKind::Factor(_) => Left,
            BinaryKind::Term(_) => Left,
            BinaryKind::Bitwise(_) => Left,
            BinaryKind::Comparison(_) => Neither,
            BinaryKind::Logical(_) => Left,
            BinaryKind::Assignment(_) => Right,
        }
    }

    #[inline]
    fn power(&self) -> Power {
        match self {
            BinaryKind::Factor(_) => 9,
            BinaryKind::Term(_) => 8,
            BinaryKind::Bitwise(b) => match b {
                Bitwise::Shr | Bitwise::Shl => 7,
                Bitwise::And => 6,
                Bitwise::Xor => 5,
                Bitwise::Or => 4,
            },
            BinaryKind::Comparison(_) => 3,
            BinaryKind::Logical(_) => 2,
            BinaryKind::Assignment(_) => 1,
        }
        .into()
    }
}

impl BinaryKind {
    #[inline]
    pub fn from_syntax(kind: SyntaxKind) -> Option<Self> {
        Factor::from_syntax(kind)
            .map(BinaryKind::Factor)
            .or_else(|| Term::from_syntax(kind).map(BinaryKind::Term))
            .or_else(|| Bitwise::from_syntax(kind).map(BinaryKind::Bitwise))
            .or_else(|| Comparison::from_syntax(kind).map(BinaryKind::Comparison))
            .or_else(|| Logical::from_syntax(kind).map(BinaryKind::Logical))
            .or_else(|| Assignment::from_syntax(kind).map(BinaryKind::Assignment))
    }
}

pub trait BinaryExt {
    fn kind(&self) -> BinaryKind;
}

impl BinaryExt for BinaryOp {
    fn kind(&self) -> BinaryKind {
        let kind = self.syntax().kind();
        let kind = BinaryKind::from_syntax(kind).expect("Invalid binary operator");

        kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
}

impl Comparison {
    pub fn from_syntax(kind: SyntaxKind) -> Option<Self> {
        use SyntaxKind::*;
        Some(match kind {
            EQ2 => Self::Eq,
            BANG_EQ => Self::Ne,
            GT => Self::Gt,
            GT_EQ => Self::Ge,
            LT => Self::Lt,
            LT_EQ => Self::Le,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Logical {
    And,
    Or,
}

impl Logical {
    pub fn from_syntax(kind: SyntaxKind) -> Option<Self> {
        use SyntaxKind::*;

        Some(match kind {
            AMP2 => Self::And,
            PIPE2 => Self::Or,
            _ => return None,
        })
    }

    fn from_syntax_assignment(kind: SyntaxKind) -> Option<Assignment> {
        use SyntaxKind::*;

        Some(Assignment::Logical(match kind {
            AMP2_EQ => Self::And,
            PIPE2_EQ => Self::Or,
            _ => return None,
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Factor {
    Div,
    Mul,
    Rem,
}

impl Factor {
    pub fn from_syntax(kind: SyntaxKind) -> Option<Self> {
        use SyntaxKind::*;

        Some(match kind {
            SLASH => Self::Div,
            STAR => Self::Mul,
            PERCENT => Self::Rem,
            _ => return None,
        })
    }

    fn from_syntax_assignment(kind: SyntaxKind) -> Option<Assignment> {
        use SyntaxKind::*;

        Some(Assignment::Factor(match kind {
            SLASH_EQ => Self::Div,
            STAR_EQ => Self::Mul,
            PERCENT_EQ => Self::Rem,
            _ => return None,
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Term {
    Add,
    Sub,
}

impl Term {
    pub fn from_syntax(kind: SyntaxKind) -> Option<Self> {
        use SyntaxKind::*;

        Some(match kind {
            PLUS => Self::Add,
            MINUS => Self::Sub,
            _ => return None,
        })
    }

    fn from_syntax_assignment(kind: SyntaxKind) -> Option<Assignment> {
        use SyntaxKind::*;

        Some(Assignment::Term(match kind {
            PLUS_EQ => Self::Add,
            MINUS_EQ => Self::Sub,
            _ => return None,
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bitwise {
    And,
    Or,
    Xor,
    Shr,
    Shl,
}

impl Bitwise {
    pub fn from_syntax(kind: SyntaxKind) -> Option<Self> {
        use SyntaxKind::*;

        Some(match kind {
            AMP => Self::And,
            PIPE => Self::Or,
            CARET => Self::Xor,
            LT2 => Self::Shl,
            GT2 => Self::Shr,
            _ => return None,
        })
    }

    fn from_syntax_assignment(kind: SyntaxKind) -> Option<Assignment> {
        use SyntaxKind::*;

        Some(Assignment::Bitwise(match kind {
            AMP_EQ => Self::And,
            PIPE_EQ => Self::Or,
            CARET_EQ => Self::Xor,
            LT2_EQ => Self::Shl,
            GT2_EQ => Self::Shr,
            _ => return None,
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Assignment {
    Assign,
    Logical(Logical),
    Factor(Factor),
    Term(Term),
    Bitwise(Bitwise),
}

impl Assignment {
    pub fn from_syntax(kind: SyntaxKind) -> Option<Self> {
        // use SyntaxKind::*;

        if kind == SyntaxKind::EQ {
            Some(Self::Assign)
        } else {
            Bitwise::from_syntax_assignment(kind)
                .or_else(|| Term::from_syntax_assignment(kind))
                .or_else(|| Factor::from_syntax_assignment(kind))
                .or_else(|| Logical::from_syntax_assignment(kind))
        }
    }
}
