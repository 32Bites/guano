use crate::ast::prelude::*;

pub mod new {
    use guano_common::{
        nom::{branch::alt, combinator::not, sequence::preceded},
        rowan::ast::AstNode,
    };
    use guano_syntax::{
        parser::{punctuation::*, wrap, Input, Res},
        SyntaxKind,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum UnaryOperator {
        Negate,
        Not,
    }

    impl UnaryOperator {
        pub fn from_op(kind: SyntaxKind) -> Self {
            match kind {
                SyntaxKind::BANG => Self::Not,
                SyntaxKind::MINUS => Self::Negate,
                _ => unreachable!(),
            }
        }

        pub fn power(&self) -> ((), u32) {
            ((), 19)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BinaryOperator {
        Add(bool),
        Sub(bool),
        Mul(bool),
        Div(bool),
        Rem(bool),
        Shl(bool),
        Shr(bool),
        LogOr(bool),
        LogAnd(bool),
        BitOr(bool),
        BitAnd(bool),
        BitXor(bool),
        Eq,
        Ne,
        Ge,
        Gt,
        Le,
        Lt,
        Assign,
    }

    impl BinaryOperator {
        pub fn from_op(kind: SyntaxKind) -> Self {
            use SyntaxKind::*;
            match kind {
                PLUS => Self::Add(false),
                PLUS_EQ => Self::Add(true),

                MINUS => Self::Sub(false),
                MINUS_EQ => Self::Sub(true),

                STAR => Self::Mul(false),
                STAR_EQ => Self::Mul(true),

                SLASH => Self::Div(false),
                SLASH_EQ => Self::Div(true),

                PERCENT => Self::Rem(false),
                PERCENT_EQ => Self::Rem(true),

                CARET => Self::BitXor(false),
                CARET_EQ => Self::BitXor(true),

                EQ => Self::Assign,
                EQ2 => Self::Eq,

                BANG_EQ => Self::Ne,

                RIGHT_ANGLE => Self::Gt,
                GT_EQ => Self::Ge,
                SHR => Self::Shr(false),
                SHR_EQ => Self::Shr(true),

                LEFT_ANGLE => Self::Lt,
                LT_EQ => Self::Le,
                SHL => Self::Shl(false),
                SHL_EQ => Self::Shl(true),

                PIPE => Self::BitOr(false),
                PIPE_EQ => Self::BitOr(true),
                PIPE2 => Self::LogOr(false),
                PIPE2_EQ => Self::LogOr(true),

                AMP => Self::BitAnd(false),
                AMP_EQ => Self::BitAnd(true),
                AMP2 => Self::LogOr(false),
                AMP2_EQ => Self::LogOr(true),
                _ => unreachable!(),
            }
        }

        pub fn is_assignment(&self) -> bool {
            match self {
                Self::Assign => true,
                Self::Add(b)
                | Self::Sub(b)
                | Self::Mul(b)
                | Self::Div(b)
                | Self::Rem(b)
                | Self::Shl(b)
                | Self::Shr(b)
                | Self::LogOr(b)
                | Self::LogAnd(b)
                | Self::BitOr(b)
                | Self::BitAnd(b)
                | Self::BitXor(b) => *b,
                _ => false,
            }
        }

        pub fn is_term(&self) -> bool {
            matches!(self, Self::Add(false) | Self::Sub(false))
        }

        pub fn is_factor(&self) -> bool {
            matches!(self, Self::Mul(true) | Self::Div(true) | Self::Rem(true))
        }

        pub fn is_shift(&self) -> bool {
            matches!(self, Self::Shl(false) | Self::Shr(false))
        }

        pub fn is_comparison(&self) -> bool {
            matches!(
                self,
                Self::Eq | Self::Ne | Self::Ge | Self::Gt | Self::Le | Self::Lt
            )
        }

        pub fn is_logical(&self) -> bool {
            matches!(self, Self::LogOr(false) | Self::LogAnd(false))
        }

        pub fn power(&self) -> (u32, u32) {
            match self {
                o if o.is_assignment() => (1, 2),
                o if o.is_logical() => (3, 4),
                o if o.is_comparison() => (5, 6),
                BinaryOperator::BitOr(false) => (7, 8),
                BinaryOperator::BitXor(false) => (9, 10),
                BinaryOperator::BitAnd(false) => (11, 12),
                o if o.is_shift() => (13, 14),
                o if o.is_term() => (15, 16),
                o if o.is_factor() => (17, 18),
                _ => unreachable!(),
            }
        }
    }

    /// Parse unary operators: -, !
    pub fn parse_unary<'a>(input: Input<'a>) -> Res<'a> {
        wrap(alt((bang, minus)), SyntaxKind::UNARY_OP)(input)
    }

    /// Parse factor operators: /, *, %
    pub fn parse_factor<'a>(input: Input<'a>) -> Res<'a> {
        wrap(
            alt((
                preceded(not(slash_eq), slash),
                preceded(not(star_eq), star),
                preceded(not(percent_eq), percent),
            )),
            SyntaxKind::BINARY_OP,
        )(input)
    }

    /// Parse term operators: +, -
    pub fn parse_term<'a>(input: Input<'a>) -> Res<'a> {
        wrap(
            alt((preceded(not(plus_eq), plus), preceded(not(minus_eq), minus))),
            SyntaxKind::BINARY_OP,
        )(input)
    }

    /// Parse shift operators: <<, >>
    pub fn parse_shift<'a>(input: Input<'a>) -> Res<'a> {
        wrap(
            alt((preceded(not(shl_eq), shl), preceded(not(shr_eq), shr))),
            SyntaxKind::BINARY_OP,
        )(input)
    }

    /// Parse bitwise and: &
    pub fn parse_bit_and<'a>(input: Input<'a>) -> Res<'a> {
        wrap(
            preceded(not(alt((amp2, amp2_eq))), amp),
            SyntaxKind::BINARY_OP,
        )(input)
    }

    /// Parse bitwise xor: ^
    pub fn parse_bit_xor<'a>(input: Input<'a>) -> Res<'a> {
        wrap(preceded(not(caret_eq), caret), SyntaxKind::BINARY_OP)(input)
    }

    /// Parse bitwise or: |
    pub fn parse_bit_or<'a>(input: Input<'a>) -> Res<'a> {
        wrap(
            preceded(not(alt((pipe2, pipe2_eq))), pipe),
            SyntaxKind::BINARY_OP,
        )(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Binary {
    Eq,     // ==
    Ne,     // !=
    Gt,     // >
    Ge,     // >=
    Shr,    // >>
    Lt,     // <
    Le,     // <=
    Shl,    // <<
    BitAnd, // &
    LogAnd, // &&
    BitOr,  // |
    LogOr,  // ||
    BitXor, // ^
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    Rem,    // %
    Assignment(Assignment),
}

impl Binary {
    pub fn is_assignment(&self) -> bool {
        matches!(self, Self::Assignment(_))
    }

    pub fn name(&self) -> &'static str {
        match self {
            Binary::Eq => "Equals",
            Binary::Ne => "Not Equals",
            Binary::Gt => "Greater Than",
            Binary::Ge => "Greater Than Equals",
            Binary::Shr => "Shift Right",
            Binary::Lt => "Less Than",
            Binary::Le => "Less Than Equals",
            Binary::Shl => "Shift Left",
            Binary::BitAnd => "Bitwise And",
            Binary::LogAnd => "Logical And",
            Binary::BitOr => "Bitwise Or",
            Binary::LogOr => "Logical Or",
            Binary::BitXor => "Bitwise Xor",
            Binary::Add => "Addition",
            Binary::Sub => "Subtraction",
            Binary::Mul => "Multiplication",
            Binary::Div => "Division",
            Binary::Rem => "Remainder",
            Binary::Assignment(a) => a.name(),
        }
    }

    pub fn parse_factor(input: Span) -> Res<Spanned<Self>> {
        spanned(preceded(
            not(Assignment::parse),
            alt((
                value(Self::Div, tag("/")),
                value(Self::Mul, tag("*")),
                value(Self::Rem, tag("%")),
            )),
        ))(input)
    }

    pub fn parse_term(input: Span) -> Res<Spanned<Self>> {
        spanned(preceded(
            not(Assignment::parse),
            alt((value(Self::Sub, tag("-")), value(Self::Add, tag("+")))),
        ))(input)
    }

    pub fn parse_shift(input: Span) -> Res<Spanned<Self>> {
        spanned(preceded(
            not(Assignment::parse),
            alt((value(Self::Shl, tag("<<")), value(Self::Shr, tag(">>")))),
        ))(input)
    }

    pub fn parse_bitand(input: Span) -> Res<Spanned<Self>> {
        spanned(preceded(
            not(Assignment::parse),
            value(Self::BitAnd, preceded(not(tag("&&")), tag("&"))),
        ))(input)
    }

    pub fn parse_bitxor(input: Span) -> Res<Spanned<Self>> {
        spanned(preceded(
            not(Assignment::parse),
            value(Self::BitXor, tag("^")),
        ))(input)
    }

    pub fn parse_bitor(input: Span) -> Res<Spanned<Self>> {
        spanned(preceded(
            not(Assignment::parse),
            value(Self::BitOr, preceded(not(tag("||")), tag("|"))),
        ))(input)
    }

    pub fn parse_comparison(input: Span) -> Res<Spanned<Self>> {
        spanned(alt((
            value(Self::Ge, tag(">=")),
            value(Self::Gt, tag(">")),
            value(Self::Le, tag("<=")),
            value(Self::Lt, tag("<")),
            value(Self::Eq, tag("==")),
            value(Self::Ne, tag("!=")),
        )))(input)
    }

    pub fn parse_logand(input: Span) -> Res<Spanned<Self>> {
        spanned(preceded(
            not(Assignment::parse),
            value(Self::LogAnd, tag("&&")),
        ))(input)
    }

    pub fn parse_logor(input: Span) -> Res<Spanned<Self>> {
        spanned(preceded(
            not(Assignment::parse),
            value(Self::LogOr, tag("||")),
        ))(input)
    }

    pub fn parse_assignment(input: Span) -> Res<Spanned<Self>> {
        spanned(map(Assignment::parse, Self::Assignment))(input)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Binary::Eq => "==",
            Binary::Ne => "!=",
            Binary::Gt => ">",
            Binary::Ge => ">=",
            Binary::Shr => ">>",
            Binary::Lt => "<",
            Binary::Le => "<=",
            Binary::Shl => "<<",
            Binary::BitAnd => "&",
            Binary::LogAnd => "&&",
            Binary::BitOr => "|",
            Binary::LogOr => "||",
            Binary::BitXor => "^",
            Binary::Add => "+",
            Binary::Sub => "-",
            Binary::Mul => "*",
            Binary::Div => "/",
            Binary::Rem => "%",
            Binary::Assignment(a) => a.as_str(),
        }
    }
}

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a Binary {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        allocator.text(self.as_str())
    }
}

impl std::fmt::Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unary {
    Neg, // -
    Not, // !
}

impl Unary {
    pub fn as_str(&self) -> &'static str {
        match self {
            Unary::Neg => "-",
            Unary::Not => "!",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Unary::Neg => "Negate",
            Unary::Not => "Not",
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Unary::Neg => '-',
            Unary::Not => '!',
        }
    }

    pub fn parse(input: Span) -> Res<Spanned<Self>> {
        spanned(alt((
            value(Self::Neg, tag("-")),
            value(Self::Not, tag("!")),
        )))(input)
    }
}

impl std::fmt::Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a Unary {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        allocator.text(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Assignment {
    Assign, // =
    Add,    // +=
    Sub,    // -=
    Mul,    // *=
    Div,    // /=
    Rem,    // %=
    BitAnd, // &=
    LogAnd, // &&=
    BitOr,  // |=
    LogOr,  // ||=
    BitXor, // ^=
    Shl,    // <<=
    Shr,    // >>=
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Assignment {
    pub fn as_str(&self) -> &'static str {
        use Assignment::*;
        match self {
            Assign => "=",
            Add => "+=",
            Sub => "-=",
            Mul => "*=",
            Div => "/=",
            Rem => "%=",
            BitAnd => "&=",
            LogAnd => "&&=",
            BitOr => "|=",
            LogOr => "||=",
            BitXor => "^=",
            Shl => "<<=",
            Shr => ">>=",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Assignment::Assign => "Assign",
            Assignment::Add => "Addition Assign",
            Assignment::Sub => "Subtraction Assign",
            Assignment::Mul => "Muliplication Assign",
            Assignment::Div => "Division Assign",
            Assignment::Rem => "Remainder Assign",
            Assignment::BitAnd => "Bitwise And Assign",
            Assignment::LogAnd => "Logical And Assign",
            Assignment::BitOr => "Bitwise Or Assign",
            Assignment::LogOr => "Logical Or Assign",
            Assignment::BitXor => "Bitwise Xor Assign",
            Assignment::Shl => "Shift Left Assign",
            Assignment::Shr => "Shift Right Assign",
        }
    }

    pub fn parse(input: Span) -> Res<Self> {
        alt((
            value(Self::Assign, tag("=")),
            value(Self::Add, tag("+=")),
            value(Self::Sub, tag("-=")),
            value(Self::Mul, tag("*=")),
            value(Self::Div, tag("/=")),
            value(Self::Rem, tag("%=")),
            value(Self::BitAnd, tag("&=")),
            value(Self::LogAnd, tag("&&=")),
            value(Self::BitOr, tag("|=")),
            value(Self::LogOr, tag("||=")),
            value(Self::BitXor, tag("^=")),
            value(Self::Shl, tag("<<=")),
            value(Self::Shr, tag(">>=")),
        ))(input)
    }
}

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a Assignment {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        allocator.text(self.as_str())
    }
}
