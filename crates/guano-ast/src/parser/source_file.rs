use guano_lexer::Token;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{
    error::{ParseError, ParseResult, ToParseResult},
    function::{Function, FunctionError},
    statement::{variable::Variable, Statement, StatementError, StatementKind},
    token_stream::{Spanned, ToSpanned},
    Parse, ParseContext,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFile {
    pub global_variables: Vec<Spanned<Variable>>,
    pub functions: Vec<Function>,
}

impl std::fmt::Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for func in &self.functions {
            writeln!(f, "{func}")?;
        }
        for var in &self.global_variables {
            writeln!(f, "{var};")?;
        }
        Ok(())
    }
}

impl Parse<SourceFileError> for SourceFile {
    fn parse(context: &mut ParseContext) -> ParseResult<Self, SourceFileError> {
        let mut global_variables = vec![];
        let mut functions = vec![];

        loop {
            match &context.stream.peek::<1>()[0] {
                Some((token, span)) => match token {
                    Token::KeyLet | Token::KeyVar => {
                        context.stream.reset_peek();

                        match Statement::parse(context).to_parse_result()? {
                            Ok(Statement {
                                kind: StatementKind::Variable(variable),
                                span,
                            }) => global_variables.push(variable.to_spanned(span)),
                            Ok(_) => unreachable!(),
                            Err(_) => {}
                        }
                    }

                    Token::KeyFun => {
                        context.stream.reset_peek();

                        let function = Function::parse(context).to_parse_result()?;
                        functions.push(function);
                    }
                    _ => return Err(ParseError::unexpected_token(span.clone())),
                },
                None => break,
            }
        }

        Ok(SourceFile {
            global_variables,
            functions,
        })
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

    if true {

    }

    let new_age: float = add(first, add(second, second));

    if true {
        
    } else {

    }

    if true {

    } else if false {

    } else {
        while true {

        }

        for _ in _ {

        }
    }

    personPrint(age, name);
}

fun personPrint @ age: float, name: string {
    print("{}'s age: {}": (name, age));
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
