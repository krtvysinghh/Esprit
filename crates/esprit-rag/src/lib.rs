use anyhow::Result;
use esprit_ai::Ai;
use esprit_embeddings::embed;
use esprit_index::search;

pub fn ask(question: &str) -> Result<String> {
    let _query_embedding = embed("nomic-embed-text", question)?;

    let files = search(question)?;

    let context = files.iter().take(20).map(|f| format!("- {}", f)).collect::<Vec<_>>().join("\n");

    let prompt = format!(
        r#"You are Esprit AI.

Answer ONLY using the supplied context.

Context:
{}

Question:
{}

Answer:"#,
        context, question
    );

    Ai::new("qwen3:1.7b").ask(&prompt)
}
