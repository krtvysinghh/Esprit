use anyhow::Result;
use esprit_ai::Ai;
use esprit_embeddings::embed;
use esprit_index::search;
use std::fs;

pub fn ask(question: &str) -> Result<String> {
    let _ = embed("nomic-embed-text", question)?;

    let mut context = String::new();

    for file in search(question)?.into_iter().take(10) {
        context.push_str(&format!("\n===== {} =====\n", file));

        if let Ok(text) = fs::read_to_string(&file) {
            context.push_str(&text.chars().take(4000).collect::<String>());
        }
    }

    let prompt = format!(
        r#"You are Esprit AI.

Answer ONLY from the supplied source code.

If the answer is absent, reply:
"I couldn't find that in the indexed project."

Project Context
===============

{}

Question
========

{}

Answer:"#,
        context, question
    );

    Ai::new("qwen3:1.7b").ask(&prompt)
}
