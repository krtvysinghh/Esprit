use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub workspace: PathBuf,
    pub model: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { workspace: std::env::current_dir().unwrap(), model: "qwen3:1.7b".into() }
    }
}
