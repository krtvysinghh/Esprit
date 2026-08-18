#[derive(Debug)]
pub enum Agent {
    Search,
    Planner,
}

pub struct Plan {
    pub steps: Vec<String>,
}

pub fn create_plan(task: &str) -> Plan {
    Plan {
        steps: vec![
            format!("Analyze task: {}", task),
            "Search relevant context".to_string(),
            "Execute workflow".to_string(),
            "Return result".to_string(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planner_creates_steps() {
        assert!(!create_plan("test").steps.is_empty());
    }
}
