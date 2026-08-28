#![forbid(unsafe_code)]
use anyhow::{anyhow, Result};
use llama_cpp_2::{
    context::params::LlamaContextParams,
    llama_backend::LlamaBackend,
    llama_batch::LlamaBatch,
    model::{params::LlamaModelParams, AddBos, LlamaModel},
};
use std::{num::NonZeroU32, sync::OnceLock};

// ── Backend singleton ─────────────────────────────────────────────────────────

static BACKEND: OnceLock<LlamaBackend> = OnceLock::new();

fn backend() -> Result<&'static LlamaBackend> {
    if let Some(b) = BACKEND.get() {
        return Ok(b);
    }
    let mut b = LlamaBackend::init().map_err(|e| anyhow!("llama backend init: {e}"))?;
    b.void_logs();
    let _ = BACKEND.set(b);
    Ok(BACKEND.get().expect("backend initialized"))
}

// ── Embedding model (lazy-loaded) ─────────────────────────────────────────────

static EMBED_MODEL: OnceLock<Option<LlamaModel>> = OnceLock::new();

fn embed_model() -> Option<&'static LlamaModel> {
    EMBED_MODEL
        .get_or_init(|| {
            let path = esprit_models::active_embed_path().ok()?;
            let bk = backend().ok()?;
            let params = LlamaModelParams::default();
            LlamaModel::load_from_file(bk, &path, &params).ok()
        })
        .as_ref()
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Generate an embedding vector for `text`.
///
/// Returns `Ok(None)` if the embedding model is not installed (the caller
/// should degrade gracefully to keyword-only search rather than erroring).
pub fn embed(text: &str) -> Result<Option<Vec<f32>>> {
    let model = match embed_model() {
        Some(m) => m,
        None => return Ok(None), // not installed — degrade gracefully
    };

    let bk = backend()?;

    let ctx_params = LlamaContextParams::default()
        .with_n_ctx(Some(NonZeroU32::new(512).expect("n_ctx non-zero")))
        .with_embeddings(true);

    let mut ctx = model
        .new_context(bk, ctx_params)
        .map_err(|e| anyhow!("Embedding context failed: {e}"))?;

    let tokens = model
        .str_to_token(text, AddBos::Always)
        .map_err(|e| anyhow!("Embed tokenise failed: {e}"))?;

    if tokens.is_empty() {
        return Ok(Some(vec![]));
    }

    let mut batch = LlamaBatch::new(tokens.len(), 1);
    for (i, &token) in tokens.iter().enumerate() {
        batch
            .add(token, i as i32, &[0], i == tokens.len() - 1)
            .map_err(|e| anyhow!("Embed batch add: {e}"))?;
    }

    ctx.decode(&mut batch)
        .map_err(|e| anyhow!("Embed decode: {e}"))?;

    // Sequence-level pooled embedding (mean pool over sequence 0)
    let emb = ctx
        .embeddings_seq_ith(0)
        .map_err(|e| anyhow!("Embeddings extraction failed: {e}"))?;

    Ok(Some(emb.to_vec()))
}

/// Returns `true` if the embedding model is installed and ready.
pub fn is_available() -> bool {
    esprit_models::active_embed_path().is_ok()
}
