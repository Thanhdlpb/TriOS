use crate::agents::agent_trait::{Agent, AgentResult};
use std::collections::HashMap;

pub struct WebAgent;

impl Agent for WebAgent {
    fn name(&self) -> &str { "web" }

    fn run(&mut self, url: &str) -> AgentResult {
        // Sử dụng shell agent để gọi curl (đơn giản hóa)
        let output = std::process::Command::new("curl").args(["-s", url]).output();
        match output {
            Ok(o) => {
                let mut data = HashMap::new();
                data.insert("body".to_string(), String::from_utf8_lossy(&o.stdout).to_string());
                AgentResult { success: true, message: "OK".to_string(), data }
            }
            Err(e) => AgentResult { success: false, message: format!("curl error: {}", e), data: HashMap::new() },
        }
    }
}
