use std::fs;
use std::path::Path;

use crate::errors::Result;

pub fn create_dir(path: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn write_file(path: impl AsRef<Path>, content: &str) -> Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;
    Ok(())
}
