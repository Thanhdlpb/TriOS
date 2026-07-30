use std::process::Command;
use crate::agents::agent_trait::{Agent, AgentResult};
use std::collections::HashMap;

pub struct ShellAgent;

impl Agent for ShellAgent {
    fn name(&self) -> &str { "shell" }

    fn run(&mut self, cmd: &str) -> AgentResult {
        let output = Command::new("sh").args(["-c", cmd]).output();
        match output {
            Ok(o) => {
                let mut data = HashMap::new();
                data.insert("stdout".to_string(), String::from_utf8_lossy(&o.stdout).to_string());
                data.insert("stderr".to_string(), String::from_utf8_lossy(&o.stderr).to_string());
                AgentResult { success: true, message: "OK".to_string(), data }
            }
            Err(e) => AgentResult { success: false, message: format!("Shell error: {}", e), data: HashMap::new() },
        }
    }
}
