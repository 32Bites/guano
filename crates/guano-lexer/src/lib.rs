use std::io::{BufRead, Seek};

use parsley_rs::lexical::Lexer;
use token::Token;

pub mod token;

/* use std::{
    collections::{LinkedList, VecDeque},
    error::Error,
    io::{BufRead, Seek},
};

use character_stream::{CharacterIterator, CharacterStream};
use token::{BitSize, Token};

pub mod token;

type LexFun = fn(
    value: &mut String,
    character: char,
    next_character: Option<char>,
) -> Result<Option<Token>, Box<dyn Error>>;

fn fix_comment(value: &String) -> String {
    if value.starts_with("#$") {
        let mut chars: VecDeque<char> = value.chars().collect();
        chars.pop_front();
        chars.pop_front();
        chars.pop_back();
        chars.pop_back();

        let string: String = chars.into_iter().collect();

        string.trim().to_string()
    } else if value.starts_with('#') {
        let mut chars = value.chars();
        chars.next();

        let string: String = chars.collect();
        string.trim().to_string()
    } else {
        value.clone()
    }
}

fn lex_string_literal(
    value: &mut String,
    character: char,
    next_character: Option<char>,
) -> Result<Token, Box<dyn Error>> {
    if value.len() == 0 {
        if character == '"' {

        }
    } else {

    }
    Err("".into())
}

fn lex_char_literal(
    value: &mut String,
    character: char,
    next_character: Option<char>,
) -> Result<Option<Token>, Box<dyn Error>> {
    Err("".into())
}

fn lex_integer_literal(
    value: &mut String,
    character: char,
    next_character: Option<char>,
) -> Result<Option<Token>, Box<dyn Error>> {
    Err("".into())
}

fn lex_hexadecimal_literal(
    value: &mut String,
    character: char,
    next_character: Option<char>,
) -> Result<Option<Token>, Box<dyn Error>> {
    Err("".into())
}

fn lex_binary_literal(
    value: &mut String,
    character: char,
    next_character: Option<char>,
) -> Result<Option<Token>, Box<dyn Error>> {
    Err("".into())
}

fn lex_worded(
    value: &mut String,
    character: char,
    next_character: Option<char>,
) -> Result<Option<Token>, Box<dyn Error>> {
    let create_token = |value: &String| -> Option<Token> {
        Some(match &**value {
            "fun" => Token::KeyFun,
            "let" => Token::KeyLet,
            "var" => Token::KeyVar,
            "ret" => Token::KeyRet,
            "as" => Token::KeyAs,
            "str" => Token::PrimStr,
            "char" => Token::PrimChar,
            _ => {
                let mut chars = value.chars();
                let num_type = chars.next().unwrap();
                let chars = chars.as_str();
                let bit_size = match chars {
                    "size" => BitSize::SystemSize,
                    "8" => BitSize::Specific(8),
                    "16" => BitSize::Specific(16),
                    "32" => BitSize::Specific(32),
                    "64" => BitSize::Specific(64),
                    _ => return Some(Token::Identifier(format!("{}{}", num_type, chars))),
                };

                match num_type {
                    'i' => Token::PrimInteger(bit_size),
                    'u' => Token::PrimUnsignedInteger(bit_size),
                    'f' => {
                        if let BitSize::Specific(32) | BitSize::Specific(64) = bit_size {
                            Token::PrimFloat(bit_size)
                        } else {
                            Token::Identifier(format!("f{}", chars))
                        }
                    }
                    _ => Token::Identifier(format!("{}{}", num_type, chars)),
                }
            }
        })
    };

    if value.len() == 0 {
        if !matches!(character, '_' | 'a'..='z' | 'A'..='Z') {
            return Err("".into());
        }
    }
    value.push(character);

    match next_character {
        Some(next_character) => {
            if matches!(next_character, '_' | 'a'..='z' | 'A'..='Z' | '0'..='9') {
                Ok(None)
            } else {
                Ok(create_token(value))
            }
        }
        None => Ok(create_token(value)),
    }
}

fn lex_whitespace(
    value: &mut String,
    character: char,
    next_character: Option<char>,
) -> Result<Token, Box<dyn Error>> {
    if !character.is_whitespace() {
        Err("".into())
    } else {
        value.push(character);

        if let Some(next_character) = next_character {
            if !next_character.is_whitespace() {
                Ok(Token::Whitespace)
            } else {
                Err("".into())
            }
        } else {
            Ok(Token::Whitespace)
        }
    }
}

fn lex_single_comment(
    value: &mut String,
    character: char,
    next_character: Option<char>,
) -> Result<Token, Box<dyn Error>> {
    if value.len() == 0 {
        if character == '#' {
            if let Some('$') = next_character {
                return Err("Next character denotes a multi-line comment.".into());
            } else if let None = next_character {
                return Ok(Token::CommSingle("".into()));
            }
        } else {
            return Err("A new comment must start with a pound symbol.".into());
        }
    }
    value.push(character);

    if let Some('\n') | None = next_character {
        return Ok(Token::CommSingle(fix_comment(value)));
    }
    Err("".into())
}

fn lex_multi_comment(
    value: &mut String,
    character: char,
    next_character: Option<char>,
) -> Result<Token, Box<dyn Error>> {
    if value.len() == 0 {
        if character == '#' {
            if let Some('$') = next_character {
            } else {
                return Err("A multiline comment must start with \"#$\"".into());
            }
        } else {
            return Err("".into());
        }
    }

    value.push(character);

    if value.ends_with("$#") {
        return Ok(Token::CommMulti(fix_comment(value)));
    }

    Err("".into())
}

fn lex_single_character(
    value: &mut String,
    character: char,
    _next_character: Option<char>,
) -> Result<Token, Box<dyn Error>> {
    if value.len() != 0 {
        return Err("There are characters remaining from the previous token".into());
    }

    let token = match character {
        '{' => Token::OpenBrace,
        '}' => Token::CloseBrace,
        '[' => Token::OpenBracket,
        ']' => Token::CloseBracket,
        '(' => Token::OpenParen,
        ')' => Token::CloseParen,
        '=' => Token::Equ,
        '&' => Token::Amp,
        '@' => Token::Asp,
        ',' => Token::Com,
        ':' => Token::Col,
        ';' => Token::Sem,
        '%' => Token::Per,
        '^' => Token::Car,
        '!' => Token::Exc,
        '|' => Token::Pip,
        '~' => Token::Til,
        '+' => Token::Plu,
        '-' => Token::Min,
        '/' => Token::Sla,
        '*' => Token::Ast,
        '<' => Token::Les,
        '>' => Token::Gre,
        '.' => Token::Dot,
        _ => return Err("Character is not a single character token.".into()),
    };

    Ok(token)
}

fn lex_funcs() -> Vec<LexFun> {
    vec![
        lex_worded,
        lex_whitespace,
        lex_multi_comment,
        lex_single_comment,
        lex_single_character,
    ]
}

pub struct Lexer<Reader: BufRead + Seek> {
    stream: CharacterIterator<Reader>,
    output: Vec<Token>,
}

impl<Reader: BufRead + Seek> Lexer<Reader> {
    pub fn new(stream: Reader) -> Self {
        Self {
            stream: CharacterIterator::new(CharacterStream::new(stream, true)),
            output: vec![],
        }
    }

    pub fn from(stream: CharacterIterator<Reader>) -> Self {
        Self {
            stream,
            output: vec![],
        }
    }

    pub fn output(&self) -> &Vec<Token> {
        &self.output
    }

    pub fn clump(&mut self) -> Result<(), Box<dyn Error>> {
        let mut output: Vec<Token> = vec![];
        let mut tokens = self.output().iter().enumerate().peekable();

        for (index, token) in tokens {
            match output.last() {
                Some(_) => todo!(),
                None => todo!(),
            }
        }
        Ok(())
    }

    pub fn tokenize(&mut self) -> Result<(), Box<dyn Error>> {
        let mut token_value = "".to_string();
        let mut lex_func: Option<LexFun> = None;

        'stream: while let Some(character) = self.stream.next() {
            let character = match character {
                character_stream::CharacterStreamResult::Character(character) => character,
                character_stream::CharacterStreamResult::Failure(_, _) => unreachable!(),
            };

            let next_character = self.stream.peek().map(|c| match c {
                character_stream::CharacterStreamResult::Character(character) => character,
                character_stream::CharacterStreamResult::Failure(_, _) => unreachable!(),
            });

            // println!("{} {:?}", token_value, character);

            if let None = lex_func {
                for func in lex_funcs() {
                    match func(&mut token_value, character, next_character) {
                        Ok(token) => {
                            token_value = "".into();
                            self.output.push(token);
                            continue 'stream;
                        }
                        Err(_) => {
                            if token_value.len() > 0 {
                                lex_func = Some(func);
                                break;
                            } else {
                                continue;
                            }
                        }
                    }
                }

                if lex_func.is_none() {
                    return Err(format!(
                        "Failed to find tokenizer for character '{}', and peeked character {:?}",
                        character, next_character
                    )
                    .into());
                }
            } else if let Some(func) = lex_func {
                match func(&mut token_value, character, next_character) {
                    Ok(token) => {
                        token_value = "".into();
                        self.output.push(token);
                        lex_func = None;
                        continue 'stream;
                    }
                    Err(_) => {}
                }
            }
        }

        self.output.push(Token::EOF);

        Ok(())
    }
}

pub trait ToLexer<Reader: BufRead + Seek> {
    fn to_lexer(self) -> Lexer<Reader>;
}

impl<Reader: BufRead + Seek> ToLexer<Reader> for Reader {
    fn to_lexer(self) -> Lexer<Reader> {
        Lexer::new(self)
    }
}
*/

pub fn lexer<Reader: BufRead + Seek>(reader: Reader) -> Lexer<Token, Reader> {
    Lexer::new(reader, true, Some(Token::EOF))
}

#[cfg(test)]
mod tests {
    use std::io::BufRead;

    use super::*;
    use parsley_rs::lexical::Lexer;
    use token::*;

    #[test]
    fn test_identifier() {
        // Lexer::new();
    }

    #[test]
    fn test_single_comment() {
    }

    #[test]
    fn test_multi_comment() {
    }
}
