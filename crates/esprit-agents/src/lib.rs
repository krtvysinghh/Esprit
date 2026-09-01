#![forbid(unsafe_code)]
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum Agent {
    /// General-purpose conversational assistant.
    Chat,
    /// Expert Rust/code assistant.
    Code,
    /// Project-search oriented agent.
    Search,
}

impl Agent {
    /// Human-readable label for this agent.
    pub fn label(&self) -> &'static str {
        match self {
            Agent::Chat => "chat",
            Agent::Code => "code",
            Agent::Search => "search",
        }
    }

    fn system_prefix(&self) -> &'static str {
        match self {
            Agent::Chat => "",
            Agent::Code => {
                "You are an expert Rust engineer with deep knowledge of \
                 systems programming, async Rust, and the Esprit codebase.\n\n"
            }
            Agent::Search => "Find everything in the project related to:\n",
        }
    }
}

/// Run an agent and return its response.
pub fn run(agent: Agent, prompt: &str) -> Result<String> {
    let full_prompt = format!("{}{}", agent.system_prefix(), prompt);
    esprit_rag::ask(&full_prompt)
}

/// Run an agent and return its response plus AI metadata.
pub fn run_with_meta(agent: Agent, prompt: &str) -> Result<(String, esprit_ai::AiMeta)> {
    let full_prompt = format!("{}{}", agent.system_prefix(), prompt);
    esprit_rag::ask_with_meta(&full_prompt)
}
