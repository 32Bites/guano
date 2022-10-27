use std::{path::PathBuf, fs::read_to_string};

use guano_ast::parser::Parser;
use ptree::print_tree;

use crate::source_files::source_files;

pub fn parse(files: Option<&Vec<PathBuf>>) {
    let sources = source_files(files);
    let mut parser = Parser::new(true);

    for file_path in sources {
        let source = read_to_string(&file_path).unwrap();
        let (_file_id, result) = parser.file(file_path, source);

        match result {
            Ok(ast) => {
                // println!("{}", serde_json::to_string_pretty(ast).unwrap());
                ptree::print_tree(&serde_value::to_value(ast).unwrap()).unwrap();
            },
            Err(error) => panic!("{error}"),
        }
    }
}