use std::path::PathBuf;

pub fn source_files<'a>(files: Option<&'a Vec<PathBuf>>) -> Box<dyn Iterator<Item = PathBuf> + 'a> {
    if let Some(files) = files {
        Box::new(files.iter().cloned())
    } else {
        Box::new(glob::glob("*.gno").unwrap().filter_map(|r| r.ok()))
    }
}
