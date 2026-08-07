use anyhow::Result;
use dashmap::DashMap;
use esprit_embeddings::embed;
use esprit_vectors::{load, store};
use std::sync::OnceLock;

static CACHE: OnceLock<DashMap<String, Vec<f32>>> = OnceLock::new();

fn cache() -> &'static DashMap<String, Vec<f32>> {
    CACHE.get_or_init(DashMap::new)
}

fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0;
    let mut na = 0.0;
    let mut nb = 0.0;

    for (x, y) in a.iter().zip(b) {
        dot += x * y;
        na += x * x;
        nb += y * y;
    }

    dot / (na.sqrt() * nb.sqrt() + 1e-8)
}

pub fn index(key: &str, text: &str) -> Result<()> {
    let v = embed("nomic-embed-text", text)?;
    cache().insert(key.to_string(), v.clone());
    store(key, &v)
}

pub fn similarity(query: &str, key: &str) -> Result<Option<f32>> {
    let q = embed("nomic-embed-text", query)?;

    let v = if let Some(v) = cache().get(key) {
        v.clone()
    } else if let Some(v) = load(key)? {
        cache().insert(key.to_string(), v.clone());
        v
    } else {
        return Ok(None);
    };

    Ok(Some(cosine(&q, &v)))
}
