use std::{fs::read_to_string, path::PathBuf};

use crate::source_files::source_files;

pub fn lex_files(files: Option<&Vec<PathBuf>>) {
    let sources = source_files(files);

    for file_path in sources {
        let _file = read_to_string(&file_path).unwrap();

        println!("Tokens for {}:", file_path.display());
        /*         for (token, span) in Token::lexer(file.as_str()).to_spanned() {
            println!(
                "{}: {:#?} - {}",
                token,
                &file.as_str()[span.byte_span.clone()],
                token
                    .value()
                    .map_or("No Value".to_string(), |v| format!("{v:#?}"))
            );
            //println!("Span: {:#?}", span);
        } */
    }
}
