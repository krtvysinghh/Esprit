use std::path::{Path, PathBuf};

pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

pub fn list<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    std::fs::read_dir(path)
        .map(|rd| rd.filter_map(|e| e.ok()).map(|e| e.path()).collect())
        .unwrap_or_default()
}
