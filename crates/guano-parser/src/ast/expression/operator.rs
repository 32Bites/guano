use crate::ast::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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

impl std::fmt::Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
