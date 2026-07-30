use std::collections::HashMap;

/// Kết quả trả về từ agent
#[derive(Debug, Clone)]
pub struct AgentResult {
    pub success: bool,
    pub message: String,
    pub data: HashMap<String, String>,
}

/// Trait cho tất cả các agent
pub trait Agent: Send + Sync {
    fn name(&self) -> &str;
    fn run(&mut self, input: &str) -> AgentResult;
    fn init(&mut self) -> Result<(), String> { Ok(()) }
    fn shutdown(&mut self) -> Result<(), String> { Ok(()) }
}
