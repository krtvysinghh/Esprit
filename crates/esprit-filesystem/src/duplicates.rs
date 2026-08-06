use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn duplicates(_root: impl AsRef<Path>) -> Result<Vec<Vec<PathBuf>>> {
    Ok(Vec::new())
}
