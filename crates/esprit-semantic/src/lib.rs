use anyhow::Result;
use esprit_embeddings::embed;
use esprit_vectors::{load, store};

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
    store(key, &v)
}

pub fn similarity(query: &str, key: &str) -> Result<Option<f32>> {
    let q = embed("nomic-embed-text", query)?;

    if let Some(v) = load(key)? {
        return Ok(Some(cosine(&q, &v)));
    }

    Ok(None)
}
