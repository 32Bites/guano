use std::fs::File;
use guano_common::{rowan::ast::AstNode, serde::Serialize};
use line_col::LineColLookup;

fn main() {
    let source = include_str!("../main.guano");
    /* println!("Hit enter to parse");
    stdin().read_line(&mut String::new()).unwrap();
 */
    let (context, result) = guano_ast::parse_file(source);

    for (i, error) in context.errors().iter().enumerate() {
        let span = error.span.unwrap();
        let start = u32::from(span.start()) as usize;
        let end = u32::from(span.end()) as usize;
        let lookup = LineColLookup::new(source);

        let (start_line, start_col) = lookup.get(start);
        let (end_line, end_col) = lookup.get(end);

        println!("Handled Error #{i}: {error}");
        println!("Start = {start_line}:{start_col}");
        println!("End = {end_line}:{end_col}");
    }

    match result {
        Ok(file) => {
            let range = file.syntax().text_range();
            let start = u32::from(range.start());
            let end = u32::from(range.end());

            println!("Success span: {start}..{end}");

            let mut json = File::create("syntax_tree.json").unwrap();

            let formatter = serde_json::ser::PrettyFormatter::with_indent("\t".as_bytes());
            let mut serializer = serde_json::ser::Serializer::with_formatter(&mut json, formatter);

            file.syntax().serialize(&mut serializer).unwrap();
        }
        Err(error) => println!("Unhandled error while parsing: {error}"),
    }
    println!("Remaining input: {:?}", context.remaining());
/*     println!("Hit enter to close");
    stdin().read_line(&mut String::new()).unwrap(); */
}
