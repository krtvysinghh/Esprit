use anyhow::Result;

pub enum Agent {
    Chat,
    Code,
    Search,
}

pub fn run(agent: Agent, prompt: &str) -> Result<String> {
    match agent {
        Agent::Chat => esprit_rag::ask(prompt),
        Agent::Code => esprit_rag::ask(&format!("You are an expert Rust engineer.\n\n{}", prompt)),
        Agent::Search => esprit_rag::ask(&format!("Find everything related to:\n{}", prompt)),
    }
}
