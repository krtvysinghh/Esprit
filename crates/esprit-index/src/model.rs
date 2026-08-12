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
    pub snippet: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SearchFilters {
    pub extension: Option<String>,
    pub path_contains: Option<String>,
    pub min_size: Option<u64>,
    pub max_size: Option<u64>,
    pub modified_after: Option<std::time::SystemTime>,
    pub modified_before: Option<std::time::SystemTime>,
}

#[derive(Debug, Clone)]
pub struct FileRelation {
    pub source: PathBuf,
    pub target: PathBuf,
    pub relation: String,
}

#[derive(Debug, Clone, Default)]
pub struct IndexHealth {
    pub indexed_files: usize,
    pub missing_files: usize,
    pub stale_files: usize,
    pub healthy: bool,
}
