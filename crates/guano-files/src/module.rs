use std::{io, path::PathBuf};

use crate::file::{File, FileId, Files};

#[derive(Debug, Clone)]
pub enum Module {
    File {
        file_id: FileId,
        file: File,
    },
    Dir {
        path: PathBuf,
        children: Vec<Module>,
    },
}

impl Module {
    pub fn open(path: PathBuf, files: &mut Files) -> io::Result<Self> {
        let metadata = path.metadata()?;

        if metadata.is_dir() {
            let mut children = vec![];
            for child in path.read_dir()?.filter_map(|r| r.ok()) {
                let metadata = child.metadata()?;
                if metadata.is_file() {
                    if child.path().extension().and_then(|o| o.to_str())
                        != Some(super::FILE_EXTENSION)
                    {
                        continue;
                    }
                }

                children.push(Module::open(child.path(), files)?);
            }

            Ok(Module::Dir { path, children })
        } else {
            let file = File::open(path)?;
            let file_id = files.add(file.clone())?;
            Ok(Module::File { file, file_id })
        }
    }
}
