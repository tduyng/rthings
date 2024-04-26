use std::{fs, path::PathBuf};

pub struct DirEntry {
    pub path: PathBuf,
    pub is_dir: bool,
}

impl DirEntry {
    pub fn new(path: PathBuf, is_dir: bool) -> Self {
        DirEntry { path, is_dir }
    }

    pub fn entries(dir: &PathBuf, root: &PathBuf) -> Vec<String> {
        let mut entries = Vec::new();

        if let Ok(entries_iter) = fs::read_dir(dir) {
            for entry in entries_iter {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let is_dir = path.is_dir();

                    let relative_path = path.strip_prefix(root).unwrap_or(&path);
                    let formatted_path = relative_path.to_string_lossy().into_owned();

                    if is_dir {
                        entries.extend(Self::entries(&path, root));
                    } else {
                        entries.push(formatted_path);
                    }
                }
            }
        }
        entries
    }
}
