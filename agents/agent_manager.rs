use std::collections::HashMap;
use crate::agents::agent_trait::{Agent, AgentResult};

pub struct AgentManager {
    agents: HashMap<String, Box<dyn Agent>>,
}

impl AgentManager {
    pub fn new() -> Self {
        Self { agents: HashMap::new() }
    }

    pub fn register(&mut self, agent: Box<dyn Agent>) {
        self.agents.insert(agent.name().to_string(), agent);
    }

    pub fn run_agent(&mut self, name: &str, input: &str) -> Option<AgentResult> {
        if let Some(agent) = self.agents.get_mut(name) {
            Some(agent.run(input))
        } else {
            None
        }
    }

    pub fn list_agents(&self) -> Vec<String> {
        self.agents.keys().cloned().collect()
    }
}
