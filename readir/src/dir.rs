use crate::Result;
use core::fmt;
use std::{fs, path::PathBuf};

pub struct DirEntry {
    pub root: PathBuf,
    pub child_paths: Vec<PathBuf>,
}

impl DirEntry {
    pub fn new(root: PathBuf) -> Result<Self> {
        let mut child_paths = Vec::new();
        DirEntry::read_entries(&mut child_paths, &root, &root)?;
        Ok(DirEntry { root, child_paths })
    }

    fn read_entries(entries: &mut Vec<PathBuf>, dir: &PathBuf, root: &PathBuf) -> Result<()> {
        if let Ok(entries_iter) = fs::read_dir(dir) {
            for entry in entries_iter {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let is_dir = path.is_dir();

                    entries.push(path.clone());
                    if is_dir {
                        DirEntry::read_entries(entries, &path, root)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl fmt::Display for DirEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for path in &self.child_paths {
            writeln!(
                f,
                "{}",
                path.strip_prefix(&self.root).unwrap_or(path).display()
            )?;
        }

        Ok(())
    }
}
