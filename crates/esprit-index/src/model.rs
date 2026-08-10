use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct IndexedFile {
    pub path: PathBuf,
    pub size: u64,
    pub modified: u64,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: PathBuf,
    pub score: f32,
}
