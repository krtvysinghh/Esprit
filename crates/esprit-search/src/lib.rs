use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn find(name: &str, root: impl AsRef<Path>) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_name().to_string_lossy().contains(name))
        .map(|e| e.into_path())
        .collect()
}
