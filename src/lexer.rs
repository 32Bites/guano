use std::{fs::read_to_string, path::PathBuf};

use guano_lexer::{logos::Logos, ToSpanned, Token};

pub fn lex_files(files: Option<&Vec<PathBuf>>) {
    let files: Box<dyn Iterator<Item = PathBuf>> = if let Some(files) = files {
        Box::new(files.iter().cloned())
    } else {
        Box::new(glob::glob("*.gno").unwrap().filter_map(|r| r.ok()))
    };

    for file_path in files {
        let file = read_to_string(&file_path).unwrap();

        println!("Tokens for {}:", file_path.display());
        for (token, span) in Token::lexer(file.as_str()).to_spanned() {
            println!(
                "{}: {:#?} - {}",
                token,
                &file.as_str()[span.byte_span.clone()],
                token
                    .value()
                    .map_or("No Value".to_string(), |v| format!("{v:#?}"))
            );
            //println!("Span: {:#?}", span);
        }
    }
}
