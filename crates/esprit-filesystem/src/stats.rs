use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Stats {
    pub files: u64,
    pub folders: u64,
    pub total_bytes: u64,
    pub extensions: HashMap<String, u64>,
}

impl Stats {
    pub fn scan(root: impl AsRef<Path>) -> Result<Self> {
        let mut stats = Stats {
            files: 0,
            folders: 0,
            total_bytes: 0,
            extensions: HashMap::new(),
        };

        for entry in WalkDir::new(root) {
            let entry = entry?;

            if entry.file_type().is_dir() {
                stats.folders += 1;
                continue;
            }

            stats.files += 1;

            let meta = entry.metadata()?;
            stats.total_bytes += meta.len();

            if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
                *stats.extensions.entry(ext.to_lowercase()).or_insert(0) += 1;
            }
        }

        Ok(stats)
    }
}
