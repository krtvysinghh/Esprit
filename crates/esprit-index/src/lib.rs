use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct IndexedFile {
    pub path: PathBuf,
    pub size: u64,
}

pub fn index(root: impl AsRef<Path>) -> Result<Vec<IndexedFile>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(root) {
        let entry = entry?;

        if !entry.file_type().is_file() {
            continue;
        }

        files.push(IndexedFile {
            path: entry.path().to_path_buf(),
            size: entry.metadata()?.len(),
        });
    }

    Ok(files)
}
