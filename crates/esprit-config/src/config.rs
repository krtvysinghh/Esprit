use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ai_model: String,
    pub workspace: PathBuf,
    pub threads: usize,
    pub color: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ai_model: "qwen3:1.7b".into(),
            workspace: std::env::current_dir().unwrap(),
            threads: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4),
            color: true,
        }
    }
}

impl Config {
    fn config_path() -> Result<PathBuf> {
        let dirs = ProjectDirs::from("dev", "esprit", "esprit")
            .ok_or_else(|| anyhow::anyhow!("unable to determine config directory"))?;

        Ok(dirs.config_dir().join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        if !path.exists() {
            let cfg = Self::default();
            cfg.save()?;
            return Ok(cfg);
        }

        let text = fs::read_to_string(path)?;
        Ok(toml::from_str(&text)?)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, toml::to_string_pretty(self)?)?;

        Ok(())
    }
}
