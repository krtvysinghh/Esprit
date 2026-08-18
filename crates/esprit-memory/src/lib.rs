use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Memory {
    pub key: String,
    pub value: String,
}

fn memory_file() -> PathBuf {
    PathBuf::from(".esprit").join("memory.json")
}

pub fn remember(key: impl Into<String>, value: impl Into<String>) -> anyhow::Result<()> {
    let path = memory_file();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut memories: Vec<(String, String)> = recall(1000)?;

    memories.push((key.into(), value.into()));

    fs::write(path, serde_json::to_string_pretty(&memories)?)?;

    Ok(())
}

pub fn recall(limit: usize) -> anyhow::Result<Vec<(String, String)>> {
    let path = memory_file();

    if !path.exists() {
        return Ok(Vec::new());
    }

    let data = fs::read_to_string(path)?;

    let memories: Vec<(String, String)> = serde_json::from_str(&data)?;

    Ok(memories.into_iter().take(limit).collect())
}

pub fn save(memory: Memory) -> anyhow::Result<()> {
    remember(memory.key, memory.value)
}

pub fn load() -> anyhow::Result<Vec<(String, String)>> {
    recall(1000)
}
