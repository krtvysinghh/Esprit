use anyhow::Result;
use esprit_agents::{Agent, run};

pub fn code_review(prompt: &str) -> Result<String> {
    run(Agent::Code, &format!("Review this code:\n\n{}", prompt))
}

pub fn explain(prompt: &str) -> Result<String> {
    run(Agent::Chat, &format!("Explain:\n\n{}", prompt))
}

pub fn search(prompt: &str) -> Result<String> {
    run(Agent::Search, prompt)
}
