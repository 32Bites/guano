use guano_lexer::Token;
use thiserror::Error;

use super::{
    error::{ParseError, ParseResult, ToParseResult},
    function::{Function, FunctionError},
    identifier::Identifier,
    statement::{variable::Variable, Statement, StatementError},
    Parse, ParseContext,
};

#[derive(Debug, Clone)]
pub enum SourceItem {
    Variable(Variable),
    Function(Function),
}

impl SourceItem {
    pub fn identifier(&self) -> &Identifier {
        match self {
            SourceItem::Variable(v) => &v.identifier,
            SourceItem::Function(f) => &f.name,
        }
    }
}

impl std::fmt::Display for SourceItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceItem::Variable(v) => write!(f, "{v};"),
            SourceItem::Function(fun) => fun.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub source_items: Vec<SourceItem>,
}

impl std::fmt::Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.source_items {
            writeln!(f, "{item}")?;
        }
        Ok(())
    }
}

impl Parse<SourceFileError> for SourceFile {
    fn parse(context: &mut ParseContext) -> ParseResult<Self, SourceFileError> {
        let mut source_items = vec![];

        loop {
            match &context.stream.peek::<1>()[0] {
                Some((token, span)) => match token {
                    Token::KeyLet | Token::KeyVar => {
                        context.stream.reset_peek();

                        if let Statement::Variable(variable) =
                            Statement::parse(context).to_parse_result()?
                        {
                            source_items.push(SourceItem::Variable(variable));
                        } else {
                            unreachable!()
                        }
                    }

                    Token::KeyFun => {
                        context.stream.reset_peek();

                        let function = Function::parse(context).to_parse_result()?;
                        source_items.push(SourceItem::Function(function));
                    }
                    _ => return Err(ParseError::unexpected_token(span.clone())),
                },
                None => break,
            }
        }

        Ok(SourceFile { source_items })
    }
}

#[derive(Error, Debug)]
pub enum SourceFileError {
    #[error("{0}")]
    StatementError(#[from] StatementError),
    #[error("{0}")]
    FunctionError(#[from] FunctionError),
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn test_source() {
        let source = r#"let global_name = "This Global Variable Was Created By Noah";

fun main @ args: ([]string, uint) {
    let name: string = "Noah";
    let first: float = 11.0;
    let second: uint = 6;
    let age: float = add(first, second);
    let new_age: float = add(first, add(second, second));

    personPrint(age, name);
}

fun personPrint @ age: float, name: string {
    print("{}'s age: {}": (name, age)); ## I need to implement format expression parsing.
}

fun add: float @ first: float, second: uint {
    return first + second as float;
}

let oops = "";"#;
        let mut parser = Parser::new(false);
        let (file_id, result) = parser.file("", source);
        match result {
            Ok(source) => println!("{source}"),
            Err(error) => {
                println!("Error: {error}");
                if let Some(span) = error.span() {
                    println!("Error string: {:#?}", &source[span.clone()]);
                    let start = parser.files.location(file_id, span.start as u32);
                    let end = parser
                        .files
                        .location(file_id, span.end.saturating_sub(1) as u32);

                    if let (Ok(start), Ok(end)) = (start, end) {
                        println!(
                            "Error start location - line: {}; col: {}",
                            start.line, start.column
                        );
                        println!(
                            "Error end location - line: {}; col: {}",
                            end.line, end.column
                        );
                    }
                }
            }
        }
    }
}
