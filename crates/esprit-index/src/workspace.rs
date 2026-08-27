use anyhow::Result;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub fn get_workspace_hash() -> String {
    let path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    esprit_utils::sha256(path.to_string_lossy().as_bytes())[..16].to_string()
}

pub fn workspace_dir(hash: &str) -> Result<PathBuf> {
    let dirs = ProjectDirs::from("dev", "esprit", "esprit")
        .ok_or_else(|| anyhow::anyhow!("unable to determine data directory"))?;
    let dir = dirs.data_dir().join("workspaces").join(hash);
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn all_workspaces() -> Result<Vec<PathBuf>> {
    let dirs = ProjectDirs::from("dev", "esprit", "esprit")
        .ok_or_else(|| anyhow::anyhow!("unable to determine data directory"))?;
    let mut out = Vec::new();
    let w_dir = dirs.data_dir().join("workspaces");
    if w_dir.exists() {
        for entry in fs::read_dir(w_dir)?.flatten() {
            out.push(entry.path());
        }
    }
    Ok(out)
}
