use anyhow::Result;
use esprit_ai::Ai;
use esprit_embeddings::embed;
use esprit_index::search;
use esprit_memory::{recall, remember};
use esprit_vectors::{nearest, store};
use std::fs;

const CONTEXT_CHARS: usize = 3_500;
const MAX_CONTEXT_FILES: usize = 8;

/// Ask a question against the indexed project, using hybrid retrieval
/// (keyword search + semantic vector search where embeddings are available).
pub fn ask(question: &str) -> Result<String> {
    let (answer, _meta) = ask_with_meta(question)?;
    Ok(answer)
}

/// Ask and return the answer plus AI metadata (token count, duration).
pub fn ask_with_meta(question: &str) -> Result<(String, esprit_ai::AiMeta)> {
    // ── 1. Embed the query (best-effort; fall back to keyword-only on failure) ──
    let query_vec = embed(question).ok().flatten();

    // ── 2. Semantic nearest-neighbour search (if we have an embedding) ─────────
    let mut sem_paths: Vec<String> = Vec::new();
    if let Some(ref qv) = query_vec {
        if esprit_vectors::count().unwrap_or(0) > 0 {
            sem_paths = nearest(qv, MAX_CONTEXT_FILES)
                .unwrap_or_default()
                .into_iter()
                .map(|(k, _)| k)
                .collect();
        }
    }

    // ── 3. Full-text (BM25) keyword search ─────────────────────────────────────
    let kw_paths = search(question).unwrap_or_default();

    // ── 4. Merge, deduplicate, prefer semantic hits first ──────────────────────
    let mut paths: Vec<String> = sem_paths;
    for p in kw_paths {
        if !paths.contains(&p) {
            paths.push(p);
        }
    }
    paths.truncate(MAX_CONTEXT_FILES);

    // ── 5. Build context (with token-budget per file) ──────────────────────────
    let mut context = String::new();
    for file in &paths {
        context.push_str(&format!("\n===== {} =====\n", file));
        if let Ok(text) = fs::read_to_string(file) {
            let snippet: String = text.chars().take(CONTEXT_CHARS).collect();
            context.push_str(&snippet);
        }
    }

    // ── 6. Conversation history ─────────────────────────────────────────────────
    let history = recall(5)
        .unwrap_or_default()
        .into_iter()
        .rev() // oldest first so the model sees chronological order
        .map(|(q, a)| format!("User: {q}\nAssistant: {a}"))
        .collect::<Vec<_>>()
        .join("\n\n");

    // ── 7. Build prompt ─────────────────────────────────────────────────────────
    let history_section = if history.is_empty() {
        String::new()
    } else {
        format!("# Conversation History\n\n{history}\n\n")
    };

    let context_section = if context.is_empty() {
        "No relevant project files were found for this query.".to_string()
    } else {
        format!("# Project Context\n{context}")
    };

    let prompt = format!(
        r#"You are Esprit AI, an expert assistant with access to the user's indexed project.

Answer ONLY from the supplied context. If the answer is absent, say:
"I couldn't find that in the indexed project."

{history_section}{context_section}

# Question

{question}

Answer:"#
    );

    // ── 8. Query the model ──────────────────────────────────────────────────────
    let ai = Ai::default_model()?;
    let (answer, meta) = ai.ask_with_meta(&prompt)?;

    // ── 9. Persist to memory & store embedding for future semantic recall ───────
    let _ = remember(question, &answer);
    if let Some(av) = embed(&answer).ok().flatten() {
        let key = format!("answer:{}", esprit_utils::sha256(question.as_bytes()));
        let _ = store(&key, &av);
    }

    Ok((answer, meta))
}

/// Return only the list of source file paths that would be used as context
/// for the given question — useful for attribution display.
pub fn source_files(question: &str) -> Result<Vec<String>> {
    let query_vec = embed(question).ok().flatten();

    let mut sem_paths: Vec<String> = Vec::new();
    if let Some(ref qv) = query_vec {
        if esprit_vectors::count().unwrap_or(0) > 0 {
            sem_paths = nearest(qv, MAX_CONTEXT_FILES)
                .unwrap_or_default()
                .into_iter()
                .map(|(k, _)| k)
                .collect();
        }
    }

    let kw_paths = search(question).unwrap_or_default();
    let mut paths = sem_paths;
    for p in kw_paths {
        if !paths.contains(&p) {
            paths.push(p);
        }
    }
    paths.truncate(MAX_CONTEXT_FILES);
    Ok(paths)
}
