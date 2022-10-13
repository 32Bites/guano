use logos::{Lexer, Logos};

/// Represents a lexical token.
#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(subpattern decimal = r"[0-9][_0-9]*")]
pub enum Token {
    // Keywords
    /// Function definition
    #[token("fun")]
    KeyFun,

    /// Constant definition
    #[token("let")]
    KeyLet,

    /// Variable definition
    #[token("var")]
    KeyVar,

    /// Nil value
    #[token("nil")]
    KeyNil,

    /// Return keyword
    #[token("ret")]
    KeyRet,

    /// Type cast keyword
    #[token("as")]
    KeyAs,

    // Primitive Types
    /// String type
    #[token("str")]
    PrimStr,

    /// Char type
    #[token("char")]
    PrimChar,

    /// Unsigned integer type
    /*     #[regex(r"u(\d+|size)", bitsize)]
    PrimUnsignedInteger(BitSize), */
    #[token("uint")]
    PrimUnsignedInteger,

    /// Signed integer type
    /*     #[regex(r"i(\d+|size)", bitsize)]
    PrimInteger(BitSize), */
    #[token("int")]
    PrimInteger,

    /// Floating point type
    /*     #[regex(r"f(\d+|size)", bitsize)]
    PrimFloat(BitSize), */
    #[token("float")]
    PrimFloat,

    /// Boolean type
    #[token("boolean")]
    PrimBool,

    // Literals
    // The in-memory value of these literals will not
    // be parsed in the lexer. That is for the parser.
    // However, there is an exception: character and string literals.
    /// Signed or unsigned integer literal
    #[regex(r"(?&decimal)", store_string)]
    LitInteger(String),

    /// Floating point literal
    #[regex(r"(?&decimal)\.(?&decimal)", store_string)]
    LitFloat(String),

    /// String literal
    #[regex(r#""(?:[^"\\\n]|\\.)*""#, text_literal)]
    LitString(String),

    /// Char literal
    #[regex(r"'(?:[^'\\\n]|\\.)*'", text_literal)]
    LitChar(String),

    /// Represents a hexadecimal value.
    /// What exact type is held will be infered base upon the context within the parser.
    #[regex(r"0[xX][0-9A-Fa-f][_0-9A-Fa-f]*", remove_first_two)]
    LitHex(String),

    /// Represents a binary value.
    /// What exact type is infered by the parser.
    #[regex(r"0[bB][01][_01]*", remove_first_two)]
    LitBin(String),

    /// Boolean Literal
    #[regex(r"false|true", |lex| if lex.slice() == "true" {true} else {false})]
    LitBool(bool),

    // General
    /// Whitespace
    #[regex(r"\s", logos::skip)]
    Whitespace,

    /// Identifier
    #[regex(r"[A-Za-z_][0-9A-Za-z_]*", store_string)]
    Identifier(String),

    // Comments
    /// Single line comment
    #[regex(r"##[^\n]*", remove_first_two)]
    CommSingle(String),

    /// Multi line comment
    #[regex(r"#\$", multiline_comment)]
    CommMulti((String, Vec<usize>)),

    // Single Character Tokens
    /// {
    #[token("{")]
    OpenBrace,

    /// Newline
    #[token("\n")]
    Newline,

    /// }
    #[token("}")]
    CloseBrace,

    /// [
    #[token("[")]
    OpenBracket,

    /// ]
    #[token("]")]
    CloseBracket,

    /// (
    #[token("(")]
    OpenParen,

    /// )
    #[token(")")]
    CloseParen,

    /// =
    #[token("=")]
    Equals,

    /// &
    #[token("&")]
    Ampersand,

    /// @
    #[token("@")]
    Asperand,

    /// ,
    #[token(",")]
    Comma,

    /// :
    #[token(":")]
    Colon,

    /// ;
    #[token(";")]
    Semicolon,

    /// %
    #[token("%")]
    Percent,

    /// ^
    #[token("^")]
    Caret,

    /// !
    #[token("!")]
    Exclamation,

    /// |
    #[token("|")]
    Pipe,

    /// ~
    #[token("~")]
    Tilda,

    /// +
    #[token("+")]
    Plus,

    /// -
    #[token("-")]
    Minus,

    /// /
    #[token("/")]
    Slash,

    /// *
    #[token("*")]
    Asterisk,

    /// <
    #[token("<")]
    LessThan,

    /// >
    #[token(">")]
    GreaterThan,

    /// .
    #[token(".")]
    Period,

    /// Error
    #[error]
    Error,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Token::KeyFun => "Function Keyword",
            Token::KeyLet => "Let Keyword",
            Token::KeyVar => "Var Keyword",
            Token::KeyRet => "Return Keyword",
            Token::KeyAs => "Type Cast Keyword",
            Token::KeyNil => "Nil Keyword",
            Token::PrimStr => "String Type",
            Token::PrimChar => "Char Type",
            Token::PrimBool => "Bool Type",
            /*             Token::PrimUnsignedInteger(b) => todo!(),
            Token::PrimInteger(_) => todo!(),
            Token::PrimFloat(_) => todo!(), */
            Token::PrimUnsignedInteger => "Unsigned Integer Type",
            Token::PrimInteger => "Signed Integer Type",
            Token::PrimFloat => "Floating Point Type",
            Token::LitInteger(_) => "Integer Literal",
            Token::LitFloat(_) => "Floating Point Literal",
            Token::LitString(_) => "String Literal",
            Token::LitChar(_) => "Character Literal",
            Token::LitHex(_) => "Hexadecimal Literal",
            Token::LitBin(_) => "Binary Literal",
            Token::LitBool(_) => "Boolean Literal",
            Token::Whitespace => "Whitespace",
            Token::Identifier(_) => "Identifier",
            Token::CommSingle(_) => "Monoline Comment",
            Token::CommMulti(_) => "Multiline Comment",
            Token::OpenBrace => "Opening Brace",
            Token::Newline => "Newline",
            Token::CloseBrace => "Closing Brace",
            Token::OpenBracket => "Opening Bracket",
            Token::CloseBracket => "Closing Bracket",
            Token::OpenParen => "Opening Parenthesis",
            Token::CloseParen => "Closing Parenthesis",
            Token::Equals => "Equals",
            Token::Ampersand => "Ampersand",
            Token::Asperand => "Asperand",
            Token::Comma => "Comma",
            Token::Colon => "Colon",
            Token::Semicolon => "Semicolon",
            Token::Percent => "Percent",
            Token::Caret => "Caret",
            Token::Exclamation => "Exclamation",
            Token::Pipe => "Pipe",
            Token::Tilda => "Tilda",
            Token::Plus => "Plus",
            Token::Minus => "Minus",
            Token::Slash => "Slash",
            Token::Asterisk => "Asterisk",
            Token::LessThan => "Less Than",
            Token::GreaterThan => "Greater Than",
            Token::Period => "Period",
            Token::Error => "Error",
        })
    }
}

impl Token {
    pub fn value(&self) -> Option<&str> {
        match self {
            Token::LitInteger(s)
            | Token::LitFloat(s)
            | Token::LitString(s)
            | Token::LitChar(s)
            | Token::LitHex(s)
            | Token::LitBin(s)
            | Token::Identifier(s)
            | Token::CommSingle(s)
            | Token::CommMulti((s, _)) => Some(s),
            Token::LitBool(b) => Some(if *b { "true" } else { "false" }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BitSize {
    SystemSize,
    Specific(u8), // 8, 16, 32, and 64 are the exclusive valid options.
}

impl std::fmt::Display for BitSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SystemSize => f.write_str("Pointer-sized"),
            Self::Specific(bit_size) => write!(f, "{}-bit", bit_size),
        }
    }
}

fn bitsize(lex: &mut Lexer<Token>) -> Option<BitSize> {
    Some(match &lex.slice()[1..] {
        "size" => BitSize::SystemSize,
        "8" => BitSize::Specific(8),
        "16" => BitSize::Specific(16),
        "32" => BitSize::Specific(32),
        "64" => BitSize::Specific(64),
        _ => return None,
    })
}

fn store_string(lex: &mut Lexer<Token>) -> String {
    lex.slice().to_owned()
}

fn text_literal(lex: &mut Lexer<Token>) -> Option<String> {
    let end = lex.slice().len() - 1;
    lex.slice().get(1..end).map(|s| s.to_owned())
}

fn remove_first_two(lex: &mut Lexer<Token>) -> Option<String> {
    lex.slice().get(2..).map(|s| s.to_owned())
}

fn multiline_comment(lex: &mut Lexer<Token>) -> Option<(String, Vec<usize>)> {
    let mut last_character = '$';
    let mut closed = false;

    let mut newline_indexes = vec![];

    for (index, character) in lex.remainder().chars().enumerate() {
        if character == '\n' {
            newline_indexes.push(lex.span().end);
        }

        lex.bump(character.len_utf8());

        if character == '#' && last_character == '$' && index != 0 {
            closed = true;
            break;
        }

        last_character = character;
    }

    if closed {
        let end = lex.slice().len() - 2;

        lex.slice()
            .get(2..end)
            .map(|s| (s.to_owned(), newline_indexes))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::spanned_lexer::ToSpanned;

    use super::Token;

    #[test]
    fn test_text() {
        let string = r#""Hello, world! \r\n \t \x41 \u0041 \U00000041"
        
        
        "Hello, world! \r\n \t \x41 \u0041 \U00000041" 
        ""#;
        let character = r#"'H' '"#;

        let mut string_lexer = Token::lexer(string).to_spanned();

        while let Some((token, _)) = string_lexer.next() {
            if let Token::Error = token {
                println!("Error causing string: {:#?}", string_lexer.slice());
            } else {
                println!("{:#?}", token);
            }
        }

        println!("{:#?}", string_lexer.lines());

        println!("------");

        let mut character_lexer = Token::lexer(character);

        while let Some(token) = character_lexer.next() {
            if let Token::Error = token {
                println!("Error causing string: {:#?}", character_lexer.slice());
            } else {
                println!("{:#?}", token);
            }
        }
    }

    #[test]
    fn test_comment() {
        let single = "## Hello, world!\n _ignore ## Other Comment";
        let multi = "#$ Rando!\r\n\n\n\n\nPPPEEEE  \n$$\n$$$$# \n\n#$$#\n\n###";

        let mut single_lexer = Token::lexer(single).to_spanned();

        while let Some((token, span)) = single_lexer.next() {
            dbg!(single_lexer.line_segments(&span.line_span));

            if let Token::Error = token {
                println!("Error causing string: {:#?}", single_lexer.slice());
            } else {
                println!("{:#?}", token);
            }
        }

        println!("------");

        let mut multi_lexer = Token::lexer(multi).to_spanned();

        while let Some((token, span)) = multi_lexer.next() {
            dbg!(multi_lexer.line_segments(&span.line_span));

            match token {
                Token::Error => {
                    println!("Error causing string: {:#?}", multi_lexer.slice());
                }
                Token::CommMulti((s, _)) => {
                    println!("Comment Contents: {:#?}", s);
                }
                _ => (),
            }
        }

        println!("{:#?}", multi_lexer.lines());
    }
}
