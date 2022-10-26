use logos::{Lexer, Logos, Source};

fn escape_parse(lex: &mut Lexer<Token>) -> Option<char> {
    if let Some(escape_code) = lex.remainder().chars().next() {
        lex.bump(escape_code.len_utf8());

        Some(match escape_code {
            't' => '\t',
            'r' => '\r',
            'n' => '\n',
            '0' => '\0',
            c @ ('\\' | '\'' | '\"') => c,
            _ => return None,
        })
    } else {
        None
    }
}

fn escape_byte_parse(lex: &mut Lexer<Token>) -> Option<char> {
    let hex_string = if let Some(hex_string) = lex.remainder().slice(0..2) {
        hex_string
    } else {
        return None;
    };

    lex.bump(hex_string.len());
    if let Ok(hex) = u8::from_str_radix(hex_string, 16) {
        // Only do ascii... for now.
        if hex <= 0x7F {
            return Some(hex as char);
        }
    }

    None
}

fn escape_unicode_parse<const N: usize>(lex: &mut Lexer<Token>) -> Option<char> {
    let hex_string = if let Some(hex_string) = lex.remainder().slice(0..N) {
        hex_string
    } else {
        return None;
    };

    lex.bump(hex_string.len());

    let parsed_character = u32::from_str_radix(hex_string, 16)
        .ok()
        .and_then(|hex| char::from_u32(hex));

    parsed_character
}

#[derive(Debug, Logos, PartialEq)]
pub enum Token {
    #[regex(".", |lex| lex.slice().chars().next())]
    Char(char),

    #[regex(r"\\", escape_parse)]
    Escape(char),

    #[regex(r"\\x", escape_byte_parse)]
    EscapeByte(char),

    #[regex(
        r"\\u",
        escape_unicode_parse::<4>
    )]
    EscapeLittleUnicode(char),

    #[regex(r"\\U", escape_unicode_parse::<8>)]
    EscapeBigUnicode(char),

    #[token("\n")]
    Newline,

    #[error]
    Error,
}

impl Token {
    pub fn char(&self) -> Option<char> {
        match self {
            Self::Error | Self::Newline => None,
            Self::Escape(c)
            | Self::Char(c)
            | Self::EscapeByte(c)
            | Self::EscapeBigUnicode(c)
            | Self::EscapeLittleUnicode(c) => Some(*c),
        }
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    #[test]
    fn test_escapes() {
        let s = r#"\U00000041Abc\t\000\"\o\o\\ fsefseafe"#;
        let mut tokens = super::Token::lexer(s);

        println!("{}", s);
        for token in &mut tokens {
            println!("{:#?}", token);
        }
    }
}
