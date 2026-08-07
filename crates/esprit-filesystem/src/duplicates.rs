use anyhow::Result;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

fn hash(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];

    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }

    let digest = hasher.finalize();

    Ok(digest.iter().map(|b| format!("{:02x}", b)).collect())
}

pub fn duplicates(root: impl AsRef<Path>) -> Result<Vec<Vec<PathBuf>>> {
    let mut map: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for entry in WalkDir::new(root) {
        let entry = entry?;

        if !entry.file_type().is_file() {
            continue;
        }

        let h = hash(entry.path())?;
        map.entry(h).or_default().push(entry.path().to_path_buf());
    }

    Ok(map.into_values().filter(|v| v.len() > 1).collect())
}
