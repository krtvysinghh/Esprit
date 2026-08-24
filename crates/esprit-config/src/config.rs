use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Default Ollama model to use for generation.
    pub ai_model: String,
    /// Fast/small model for quick tasks (e.g. summarization, routing).
    pub ai_model_fast: String,
    /// Deep/large model for complex reasoning or refactoring.
    pub ai_model_deep: String,
    /// Ollama service URL.
    pub ollama_url: String,
    /// Root directory of the current workspace.
    pub workspace: PathBuf,
    /// Number of rayon worker threads (0 = auto).
    pub threads: usize,
    /// Enable terminal color output.
    pub color: bool,
    /// Maximum characters of file content to include per file in RAG context.
    pub context_chars_per_file: usize,
    /// Maximum number of files to include in RAG context.
    pub max_context_files: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ai_model: std::env::var("ESPRIT_MODEL").unwrap_or_else(|_| "qwen3:1.7b".into()),
            ai_model_fast: std::env::var("ESPRIT_MODEL_FAST").unwrap_or_else(|_| "qwen3:0.6b".into()),
            ai_model_deep: std::env::var("ESPRIT_MODEL_DEEP").unwrap_or_else(|_| "llama3:8b".into()),
            ollama_url: std::env::var("OLLAMA_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:11434".into()),
            workspace: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            threads: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4),
            color: true,
            context_chars_per_file: 3_500,
            max_context_files: 8,
        }
    }
}

impl Config {
    fn dirs() -> Result<ProjectDirs> {
        ProjectDirs::from("dev", "esprit", "esprit")
            .ok_or_else(|| anyhow!("unable to determine config directory"))
    }

    fn config_path() -> Result<PathBuf> {
        Ok(Self::dirs()?.config_dir().join("config.toml"))
    }

    /// Load config from disk, creating a default one if it doesn't exist.
    /// Environment variables `ESPRIT_MODEL` and `OLLAMA_URL` always override
    /// the stored values, so users don't have to edit the file to switch models.
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        let mut cfg = if path.exists() {
            let text = fs::read_to_string(&path)?;
            toml::from_str::<Self>(&text)?
        } else {
            let cfg = Self::default();
            cfg.save()?;
            cfg
        };

        // Env vars take precedence over stored config
        if let Ok(m) = std::env::var("ESPRIT_MODEL") {
            cfg.ai_model = m;
        }
        if let Ok(u) = std::env::var("OLLAMA_URL") {
            cfg.ollama_url = u;
        }

        Ok(cfg)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, toml::to_string_pretty(self)?)?;
        Ok(())
    }

    /// Convenience: save a single field change.
    pub fn set_model(&mut self, model: impl Into<String>) -> Result<()> {
        self.ai_model = model.into();
        self.save()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskComplexity {
    Fast,
    Default,
    Deep,
}

impl Config {
    /// Routes the task to the appropriate model based on requested complexity
    pub fn route_model(&self, complexity: TaskComplexity) -> &str {
        match complexity {
            TaskComplexity::Fast => &self.ai_model_fast,
            TaskComplexity::Deep => &self.ai_model_deep,
            TaskComplexity::Default => &self.ai_model,
        }
    }
}
