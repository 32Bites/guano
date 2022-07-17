#[derive(Debug, Clone, PartialEq)]
pub enum BitSize {
    SystemSize,
    Specific(u8), // 8, 16, 32, and 64 are the exclusive valid options.
}

impl ToString for BitSize {
    fn to_string(&self) -> String {
        match self {
            Self::SystemSize => "size".into(),
            Self::Specific(bit_size) => format!("{}", bit_size),
        }
    }
}

/// Represents a lexical token.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// EOF
    EOF,

    // Keywords
    /// Function definition
    KeyFun,
    /// Constant definition
    KeyLet,
    /// Variable definition
    KeyVar,
    /// Return keyword
    KeyRet,
    /// Type cast keyword
    KeyAs,

    // Primitive Types
    /// String type
    PrimStr,
    /// Char type
    PrimChar,
    /// Unsigned integer type
    PrimUnsignedInteger(BitSize),
    /// Signed integer type
    PrimInteger(BitSize),
    /// Floating point type
    PrimFloat(BitSize),

    // Literals
    // The in-memory value of these literals will not
    // be parsed in the lexer. That is for the parser.
    /// Signed or unsigned integer literal
    LitInteger(String),
    /// Floating point literal
    LitFloat(String),
    /// String literal
    LitString(String),
    /// Char literal
    LitChar(String),
    /// Represents a hexadecimal value.
    /// What exact type is held will be infered base upon the context within the parser.
    LitHex(String),
    /// Represents a binary value.
    /// What exact type is infered by the parser.
    LitBin(String),

    // General
    /// Whitespace
    Whitespace,
    /// Identifier
    Identifier(String),

    // Comments
    /// Single line comment
    CommSingle(String),
    /// Multi line comment
    CommMulti(String),

    // Single Character Tokens
    /// {
    OpenBrace,
    /// }
    CloseBrace,
    /// [
    OpenBracket,
    /// ]
    CloseBracket,
    /// (
    OpenParen,
    /// )
    CloseParen,
    /// =
    Equ,
    /// &       
    Amp,
    /// @
    Asp,
    /// ,
    Com,
    /// :
    Col,
    /// ;
    Sem,
    /// %
    Per,
    /// ^
    Car,
    /// !
    Exc,
    /// |
    Pip,
    /// ~
    Til,
    /// +
    Plu,
    /// -
    Min,
    /// /
    Sla,
    /// *
    Ast,
    /// <
    Les,
    /// >
    Gre,
    /// .
    Dot,
}
