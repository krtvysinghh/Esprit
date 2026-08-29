#![warn(missing_debug_implementations)]
#![forbid(unsafe_code)]
use anyhow::{anyhow, bail, Result};
use directories::ProjectDirs;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf, time::Duration};

// ── Known models ─────────────────────────────────────────────────────────────

/// A model entry in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEntry {
    pub id: &'static str,
    pub display: &'static str,
    pub description: &'static str,
    pub url: &'static str,
    pub filename: &'static str,
    /// Expected file size in bytes (approx, for progress bar).
    pub size_bytes: u64,
    pub kind: ModelKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelKind {
    /// General-purpose language model.
    Llm,
    /// Embedding model (returns vectors, not text).
    Embedding,
}

/// All models Esprit knows about, downloaded from HuggingFace.
pub const REGISTRY: &[ModelEntry] = &[
    ModelEntry {
        id: "qwen3:0.6b",
        display: "Qwen3 0.6B (default, fast)",
        description: "Small but capable bilingual model. ~390 MB.",
        url: "https://huggingface.co/Qwen/Qwen3-0.6B-GGUF/resolve/main/qwen3-0.6b-q4_k_m.gguf",
        filename: "qwen3-0.6b-q4_k_m.gguf",
        size_bytes: 409_000_000,
        kind: ModelKind::Llm,
    },
    ModelEntry {
        id: "qwen3:1.7b",
        display: "Qwen3 1.7B (balanced)",
        description: "Better reasoning, more memory. ~1.1 GB.",
        url: "https://huggingface.co/Qwen/Qwen3-1.7B-GGUF/resolve/main/qwen3-1.7b-q4_k_m.gguf",
        filename: "qwen3-1.7b-q4_k_m.gguf",
        size_bytes: 1_100_000_000,
        kind: ModelKind::Llm,
    },
    ModelEntry {
        id: "nomic-embed",
        display: "Nomic Embed Text v1.5 (semantic search)",
        description: "High-quality embedding model. ~80 MB.",
        url: "https://huggingface.co/nomic-ai/nomic-embed-text-v1.5-GGUF/resolve/main/nomic-embed-text-v1.5.Q4_K_M.gguf",
        filename: "nomic-embed-text-v1.5.Q4_K_M.gguf",
        size_bytes: 80_000_000,
        kind: ModelKind::Embedding,
    },
];

// ── Paths ─────────────────────────────────────────────────────────────────────

fn dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("dev", "esprit", "esprit")
        .ok_or_else(|| anyhow!("unable to determine data directory"))
}

/// Root directory where all model files are stored.
pub fn models_dir() -> Result<PathBuf> {
    let d = dirs()?;
    let dir = d.data_dir().join("models");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Full path to a specific model file.
pub fn model_path(filename: &str) -> Result<PathBuf> {
    Ok(models_dir()?.join(filename))
}

// ── Lookup ────────────────────────────────────────────────────────────────────

/// Find a registry entry by model ID (e.g. `"qwen3:0.6b"`).
pub fn lookup(id: &str) -> Option<&'static ModelEntry> {
    REGISTRY.iter().find(|e| e.id == id)
}

/// The default LLM model entry.
pub fn default_llm() -> &'static ModelEntry {
    &REGISTRY[0]
}

/// The default embedding model entry.
pub fn default_embed() -> &'static ModelEntry {
    &REGISTRY[2]
}

// ── Status ────────────────────────────────────────────────────────────────────

/// Returns `true` if the model file exists on disk.
pub fn is_installed(entry: &ModelEntry) -> Result<bool> {
    Ok(model_path(entry.filename)?.exists())
}

/// Returns the path to the installed model, or an error with guidance.
pub fn require(entry: &ModelEntry) -> Result<PathBuf> {
    let path = model_path(entry.filename)?;
    if path.exists() {
        Ok(path)
    } else {
        bail!(
            "Model \"{}\" is not installed.\n\
             Run:  esprit model pull {}\n\
             Or:   esprit init          (to install defaults)",
            entry.display,
            entry.id
        )
    }
}

// ── Download ──────────────────────────────────────────────────────────────────

/// Download a model to the models directory with a live progress bar.
/// Resumes partial downloads automatically.
pub fn pull(entry: &ModelEntry) -> Result<PathBuf> {
    let dest = model_path(entry.filename)?;

    // Check if already complete
    if dest.exists() {
        let size = fs::metadata(&dest)?.len();
        if size >= entry.size_bytes.saturating_sub(1_000_000) {
            // within 1 MB = treat as complete
            println!("  ✓ {} is already installed.", entry.display);
            return Ok(dest);
        }
        // partial — resume
    }

    let partial = dest.with_extension("part");
    let resume_from = if partial.exists() {
        fs::metadata(&partial)?.len()
    } else {
        0
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(3600))
        .build()?;

    let mut req = client.get(entry.url);
    if resume_from > 0 {
        req = req.header("Range", format!("bytes={}-", resume_from));
    }

    let mut response = req
        .send()
        .map_err(|e| anyhow!("Download failed: {e}\nCheck your internet connection."))?;

    let status = response.status();
    if !status.is_success() && status.as_u16() != 206 {
        bail!("Server returned {status} for {}", entry.url);
    }

    let total = entry.size_bytes;

    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::with_template(
            "  {msg:.bold}  [{bar:42.cyan/black}]  {bytes}/{total_bytes}  {eta}",
        )
        .unwrap()
        .progress_chars("█▉▊▋▌▍▎▏ "),
    );
    pb.set_message(format!("Downloading {}", entry.display));
    pb.set_position(resume_from);

    let mut file = if resume_from > 0 {
        fs::OpenOptions::new().append(true).open(&partial)?
    } else {
        fs::File::create(&partial)?
    };

    let mut buf = [0u8; 65536];
    loop {
        use std::io::Read;
        let n = response.read(&mut buf)?;
        if n == 0 {
            break;
        }
        file.write_all(&buf[..n])?;
        pb.inc(n as u64);
    }

    pb.finish_and_clear();
    drop(file);

    fs::rename(&partial, &dest)?;
    println!("  ✓ {} saved to {}", entry.display, dest.display());

    Ok(dest)
}

/// Remove an installed model.
pub fn remove(entry: &ModelEntry) -> Result<()> {
    let path = model_path(entry.filename)?;
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(())
}

// ── Listing ───────────────────────────────────────────────────────────────────

/// Return all models with their installation status.
pub fn list_status() -> Result<Vec<(&'static ModelEntry, bool)>> {
    let mut out = Vec::new();
    for entry in REGISTRY {
        let installed = is_installed(entry)?;
        out.push((entry, installed));
    }
    Ok(out)
}

// ── Path resolution for AI backends ──────────────────────────────────────────

/// Find the path to the active LLM model, preferring the model ID in
/// `ESPRIT_MODEL` env var, then the default.
pub fn active_llm_path() -> Result<PathBuf> {
    let id = std::env::var("ESPRIT_MODEL").unwrap_or_else(|_| default_llm().id.to_string());
    let entry = lookup(&id).unwrap_or_else(default_llm);
    require(entry)
}

/// Find the path to the active embedding model.
pub fn active_embed_path() -> Result<PathBuf> {
    require(default_embed())
}
