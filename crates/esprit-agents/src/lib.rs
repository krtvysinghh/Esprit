#[derive(Debug, Clone)]
pub enum Agent {
    Search,
    Planner,
    Chat,
    Code,
}

pub fn run(agent: Agent, input: &str) -> anyhow::Result<String> {
    Ok(match agent {
        Agent::Search => format!("Searching: {}", input),
        Agent::Planner => format!("Planning: {}", input),
        Agent::Chat => format!("Chat: {}", input),
        Agent::Code => format!("Code: {}", input),
    })
}

pub struct Plan {
    pub steps: Vec<String>,
}

pub fn create_plan(task: &str) -> Plan {
    Plan {
        steps: vec![
            format!("Analyze {}", task),
            "Search context".into(),
            "Execute".into(),
        ],
    }
}
