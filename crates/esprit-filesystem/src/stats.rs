use anyhow::Result;
use std::{collections::HashMap, path::Path};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct FolderStats {
    pub files: u64,
    pub directories: u64,
    pub bytes: u64,
    pub extensions: HashMap<String, u64>,
}

impl FolderStats {
    pub fn scan(root: impl AsRef<Path>) -> Result<Self> {
        let mut stats = FolderStats {
            files: 0,
            directories: 0,
            bytes: 0,
            extensions: HashMap::new(),
        };

        for entry in WalkDir::new(root).follow_links(false) {
            let entry = entry?;

            if entry.file_type().is_dir() {
                stats.directories += 1;
                continue;
            }

            stats.files += 1;
            stats.bytes += entry.metadata()?.len();

            if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
                *stats.extensions.entry(ext.to_lowercase()).or_insert(0) += 1;
            }
        }

        Ok(stats)
    }
}
