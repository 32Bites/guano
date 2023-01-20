use crate::ast::{declaration::modifier::Modifier, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    Return,
    While,
    For,
    In,
    Loop,
    Break,
    Continue,
    Impl,
    Fun,
    Let,
    Var,
    Veto,
    Pub,
    Proto,
    Class,
    Module,
    Try,
    Catch,
    Throw,
    As,
    Import,
    Is,
    From,
    If,
    Else,
    Static,
    This,
    Nil,
    True,
    False,
    NaN,
    Inf,
    Float,
    Uint,
    Int,
    Char,
    String,
    Boolean,
}

impl Keyword {
    pub fn parse(input: Span) -> Res<Spanned<Self>> {
        map_opt(Iden::parse_raw, |s| {
            Keyword::from_str(&s).map(|k| Spanned::new(k, s.span().clone()))
        })(input)
    }

    pub fn is_primitive(&self) -> bool {
        use Keyword::*;
        matches!(self, Float | Uint | Int | Char | String | Boolean)
    }

    pub fn is_literal(&self) -> bool {
        use Keyword::*;
        matches!(self, Nil | False | True | Inf | NaN)
    }

    pub fn is_modifier(&self) -> bool {
        use Keyword::*;
        matches!(self, Veto | Static | Pub)
    }

    pub fn to_modifier(&self) -> Option<Modifier> {
        use Keyword::*;
        Some(match self {
            Veto => Modifier::Veto,
            Pub => Modifier::Pub,
            Static => Modifier::Static,
            _ => return None,
        })
    }

    pub fn as_str(&self) -> &'static str {
        use Keyword::*;
        match self {
            Return => "return",
            While => "while",
            For => "for",
            In => "in",
            Loop => "loop",
            Break => "break",
            Continue => "continue",
            Impl => "impl",
            Fun => "fun",
            Let => "let",
            Var => "var",
            Veto => "veto",
            Pub => "pub",
            Proto => "proto",
            Class => "class",
            Try => "try",
            Catch => "catch",
            Throw => "throw",
            As => "as",
            Import => "import",
            Is => "is",
            From => "from",
            If => "if",
            Else => "else",
            Static => "static",
            This => "this",
            Nil => "nil",
            True => "true",
            False => "false",
            NaN => "nan",
            Inf => "inf",
            Float => "float",
            Uint => "uint",
            Int => "int",
            Char => "char",
            String => "string",
            Boolean => "boolean",
            Module => "module",
        }
    }

    pub fn from_str(s: &str) -> Option<Keyword> {
        use Keyword::*;
        Some(match s {
            "return" => Return,
            "while" => While,
            "for" => For,
            "in" => In,
            "loop" => Loop,
            "break" => Break,
            "continue" => Continue,
            "impl" => Impl,
            "fun" => Fun,
            "let" => Let,
            "var" => Var,
            "veto" => Veto,
            "pub" => Pub,
            "proto" => Proto,
            "class" => Class,
            "try" => Try,
            "catch" => Catch,
            "throw" => Throw,
            "as" => As,
            "import" => Import,
            "is" => Is,
            "from" => From,
            "if" => If,
            "else" => Else,
            "static" => Static,
            "this" => This,
            "nil" => Nil,
            "true" => True,
            "false" => False,
            "nan" => NaN,
            "inf" => Inf,
            "float" => Float,
            "uint" => Uint,
            "int" => Int,
            "char" => Char,
            "string" => String,
            "boolean" => Boolean,
            "module" => Module,
            _ => return None,
        })
    }
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Spanned<Keyword> {
    pub fn to_literal(&self) -> Option<Lit> {
        Some(match self.value() {
            Keyword::Nil => Lit::new_nil(self.span()),
            Keyword::False => Lit::new_bool(false, self.span()),
            Keyword::True => Lit::new_bool(true, self.span()),
            Keyword::Inf => Lit::new_float(Float::Infinity, self.span()),
            Keyword::NaN => Lit::new_float(Float::NaN, self.span()),
            _ => return None,
        })
    }
}

impl Parser<Span, Spanned<Keyword>, NomError<Span>> for Keyword {
    fn parse(&mut self, input: Span) -> Res<Spanned<Self>> {
        verify(Self::parse, |s| s.value() == self)(input)
    }
}

impl PartialEq<&str> for Keyword {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<Iden> for Keyword {
    fn eq(&self, other: &Iden) -> bool {
        self.as_str() == other.as_str()
    }
}
