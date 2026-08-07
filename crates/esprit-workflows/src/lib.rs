use anyhow::Result;
use esprit_agents::{Agent, run};

pub fn explain(prompt: &str) -> Result<String> {
    run(Agent::Chat, prompt)
}

pub fn code_review(prompt: &str) -> Result<String> {
    run(Agent::Code, &format!("Review this code thoroughly:\n\n{}", prompt))
}

pub fn project_search(prompt: &str) -> Result<String> {
    run(Agent::Search, prompt)
}
