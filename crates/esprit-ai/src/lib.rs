use anyhow::{anyhow, bail, Result};
use llama_cpp_2::{
    context::params::LlamaContextParams,
    llama_backend::LlamaBackend,
    llama_batch::LlamaBatch,
    model::{params::LlamaModelParams, AddBos, LlamaModel},
    sampling::LlamaSampler,
};
use std::{num::NonZeroU32, sync::OnceLock};

// ── Backend (one per process) ─────────────────────────────────────────────────

static BACKEND: OnceLock<LlamaBackend> = OnceLock::new();

fn backend() -> Result<&'static LlamaBackend> {
    if let Some(b) = BACKEND.get() {
        return Ok(b);
    }
    let mut b = LlamaBackend::init().map_err(|e| anyhow!("llama backend init: {e}"))?;
    b.void_logs(); // suppress llama.cpp stderr noise
    let _ = BACKEND.set(b);
    Ok(BACKEND.get().expect("backend initialized"))
}

// ── Public types ──────────────────────────────────────────────────────────────

/// Metadata returned alongside a generation response.
#[derive(Debug, Clone)]
pub struct AiMeta {
    /// Number of tokens generated.
    pub tokens: u64,
    /// Wall-clock time for generation in milliseconds.
    pub duration_ms: u64,
}

/// Inference client backed by a local GGUF model via llama.cpp.
///
/// Loading a model is expensive (~100–500 ms depending on size).
/// Reuse the same `Ai` instance for multiple calls where possible.
pub struct Ai {
    model: LlamaModel,
    /// Maximum new tokens to generate per call.
    max_tokens: usize,
}

impl Ai {
    /// Load a model from a GGUF file path.
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let bk = backend()?;
        let params = LlamaModelParams::default();
        let model = LlamaModel::load_from_file(bk, path.as_ref(), &params)
            .map_err(|e| anyhow!("Failed to load model from {}: {e}", path.as_ref().display()))?;
        Ok(Self { model, max_tokens: 1024 })
    }

    /// Load the model identified by `ESPRIT_MODEL` env var, or the default.
    pub fn default_model() -> Result<Self> {
        let path = esprit_models::active_llm_path()?;
        Self::load(path)
    }

    /// Override the maximum number of tokens to generate.
    pub fn with_max_tokens(mut self, n: usize) -> Self {
        self.max_tokens = n;
        self
    }

    /// Generate a response to `prompt` and return the text.
    pub fn ask(&self, prompt: &str) -> Result<String> {
        Ok(self.ask_with_meta(prompt)?.0)
    }

    /// Generate a response and return text plus metadata.
    pub fn ask_with_meta(&self, prompt: &str) -> Result<(String, AiMeta)> {
        let bk = backend()?;

        // ── 1. Context ────────────────────────────────────────────────────────
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(Some(
                NonZeroU32::new(8192).expect("n_ctx must be non-zero"),
            ))
            .with_n_batch(512);

        let mut ctx = self
            .model
            .new_context(bk, ctx_params)
            .map_err(|e| anyhow!("Context creation failed: {e}"))?;

        // ── 2. Tokenise ───────────────────────────────────────────────────────
        let tokens = self
            .model
            .str_to_token(prompt, AddBos::Always)
            .map_err(|e| anyhow!("Tokenisation failed: {e}"))?;

        if tokens.is_empty() {
            bail!("Prompt produced zero tokens");
        }

        let cap = tokens.len() + self.max_tokens;
        let mut batch = LlamaBatch::new(cap, 1);

        for (i, &token) in tokens.iter().enumerate() {
            let is_last = i == tokens.len() - 1;
            batch
                .add(token, i as i32, &[0], is_last)
                .map_err(|e| anyhow!("Batch add failed: {e}"))?;
        }

        // ── 3. Prefill ────────────────────────────────────────────────────────
        ctx.decode(&mut batch)
            .map_err(|e| anyhow!("Prefill decode failed: {e}"))?;

        // ── 4. Generate ───────────────────────────────────────────────────────
        let mut sampler = LlamaSampler::chain_simple([
            LlamaSampler::top_p(0.9, 1),
            LlamaSampler::temp(0.7),
            LlamaSampler::dist(42),
        ]);
        let mut output = String::with_capacity(512);
        let mut decoder = encoding_rs::UTF_8.new_decoder();
        let mut n_cur = tokens.len();
        let start = std::time::Instant::now();
        let mut n_generated: u64 = 0;

        loop {
            let token = sampler.sample(&ctx, batch.n_tokens() - 1);
            sampler.accept(token);

            // End-of-generation check
            if self.model.is_eog_token(token) {
                break;
            }

            // Max-tokens guard
            if n_generated as usize >= self.max_tokens {
                break;
            }

            // Decode token to text piece
            if let Ok(piece) = self.model.token_to_piece(token, &mut decoder, false, None) {
                output.push_str(&piece);
            }
            n_generated += 1;

            // Build next batch (single token)
            batch.clear();
            batch
                .add(token, n_cur as i32, &[0], true)
                .map_err(|e| anyhow!("Batch add failed: {e}"))?;
            n_cur += 1;

            ctx.decode(&mut batch)
                .map_err(|e| anyhow!("Decode failed: {e}"))?;
        }

        Ok((
            output,
            AiMeta {
                tokens: n_generated,
                duration_ms: start.elapsed().as_millis() as u64,
            },
        ))
    }

    /// Check whether the model file exists (no inference performed).
    pub fn is_available() -> bool {
        esprit_models::active_llm_path().is_ok()
    }
}
