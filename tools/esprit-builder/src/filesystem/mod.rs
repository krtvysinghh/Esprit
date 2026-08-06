use camino::Utf8Path;
use std::fs;

use crate::errors::Result;

pub fn create_dir(path: impl AsRef<Utf8Path>) -> Result<()> {
    fs::create_dir_all(path.as_ref())?;
    Ok(())
}

pub fn write(path: impl AsRef<Utf8Path>, contents: &str) -> Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path.as_ref(), contents)?;
    Ok(())
}

pub fn exists(path: impl AsRef<Utf8Path>) -> bool {
    path.as_ref().exists()
}

pub fn remove(path: impl AsRef<Utf8Path>) -> Result<()> {
    let p = path.as_ref();

    if !p.exists() {
        return Ok(());
    }

    if p.is_dir() {
        fs::remove_dir_all(p)?;
    } else {
        fs::remove_file(p)?;
    }

    Ok(())
}
