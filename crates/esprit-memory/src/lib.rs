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

    let memory = Memory {
        key: key.into(),
        value: value.into(),
    };

    fs::write(path, serde_json::to_string_pretty(&memory)?)?;

    Ok(())
}

pub fn recall() -> anyhow::Result<Option<Memory>> {
    let path = memory_file();

    if !path.exists() {
        return Ok(None);
    }

    let data = fs::read_to_string(path)?;
    Ok(Some(serde_json::from_str(&data)?))
}

pub fn save(memory: Memory) -> anyhow::Result<()> {
    remember(memory.key, memory.value)
}

pub fn load() -> anyhow::Result<Option<Memory>> {
    recall()
}
