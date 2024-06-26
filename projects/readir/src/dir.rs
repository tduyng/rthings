use crate::Result;
use core::fmt;
use std::{
    collections::VecDeque,
    fs,
    path::{Path, PathBuf},
};

pub struct DirEntry {
    pub root: PathBuf,
    pub child_paths: Vec<PathBuf>,
}

impl DirEntry {
    pub fn new(root: PathBuf) -> Result<Self> {
        let mut child_paths = Vec::new();
        DirEntry::read_entries(&mut child_paths, &root)?;
        Ok(DirEntry { root, child_paths })
    }

    pub fn list_files_in_root(&self, include_nested: bool) -> Vec<&PathBuf> {
        if include_nested {
            self.child_paths.iter().filter(|p| !p.is_dir()).collect()
        } else {
            self.child_paths
                .iter()
                .filter(|p| !p.is_dir() && p.parent() == Some(&self.root))
                .collect()
        }
    }

    pub fn list_all_files(&self) -> &Vec<PathBuf> {
        &self.child_paths
    }

    fn read_entries(entries: &mut Vec<PathBuf>, dir: &PathBuf) -> Result<()> {
        if let Ok(entries_iter) = fs::read_dir(dir) {
            for entry in entries_iter {
                let entry = entry?;
                let path = entry.path();
                let is_dir = path.is_dir();

                entries.push(path.clone());
                if is_dir {
                    DirEntry::read_entries(entries, &path)?;
                }
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn read_entries_bfs(entries: &mut Vec<PathBuf>, dir: &Path) -> Result<()> {
        let mut queue = VecDeque::new();
        queue.push_back(dir.to_path_buf());

        while let Some(current_dir) = queue.pop_front() {
            if let Ok(entries_iter) = fs::read_dir(&current_dir) {
                for entry in entries_iter {
                    let entry = entry?;
                    let path = entry.path();
                    let is_dir = path.is_dir();

                    entries.push(path.clone());
                    if is_dir {
                        queue.push_back(path)
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
