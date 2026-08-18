use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Memory {
    pub key: String,
    pub value: String,
}

fn memory_file() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("esprit")
        .join("memory.json")
}

pub fn save(memory: Memory) -> anyhow::Result<()> {
    let path = memory_file();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let data = serde_json::to_string_pretty(&memory)?;
    fs::write(path, data)?;

    Ok(())
}

pub fn load() -> anyhow::Result<Option<Memory>> {
    let path = memory_file();

    if !path.exists() {
        return Ok(None);
    }

    let data = fs::read_to_string(path)?;
    Ok(Some(serde_json::from_str(&data)?))
}
