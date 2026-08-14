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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agents_have_all_modes() {
        let _ = Agent::Chat;
        let _ = Agent::Code;
        let _ = Agent::Search;
    }
}
