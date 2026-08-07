use anyhow::Result;
use esprit_ai::Ai;
use esprit_index::search;

pub fn ask(question: &str) -> Result<String> {
    let files = search(question)?;

    let context = files.iter().take(20).map(|f| format!("- {}", f)).collect::<Vec<_>>().join("\n");

    let prompt =
        format!("You are Esprit.\n\nRelevant files:\n{}\n\nQuestion:\n{}\n", context, question);

    Ai::new("qwen3:1.7b").ask(&prompt)
}
