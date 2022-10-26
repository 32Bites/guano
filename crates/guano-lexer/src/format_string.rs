use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
pub enum Token {
    #[token("{}")]
    Format,
    #[token("{{")]
    EscapeLeft,
    #[token("}}")]
    EscapeRight,
    #[regex("(?s:.)", |lex| lex.slice().chars().next())]
    Char(char),
    #[error]
    Error,
}

impl Token {
    pub fn char(&self) -> Option<char> {
        match self {
            Token::Format | Token::Error => None,
            Token::EscapeLeft => Some('{'),
            Token::EscapeRight => Some('}'),
            Token::Char(c) => Some(*c),
        }
    }
}
