use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::Mutex;
use tricore::lexer::Lexer;
use tricore::parser::Parser;
use tricore::interpreter::Interpreter;

#[derive(Debug, Clone, serde::Serialize)]
pub struct AgentInfo {
    pub name: String,
    pub status: String,
    pub last_output: String,
}

pub struct AgentRuntime {
    agents: HashMap<String, AgentInfo>,
    interpreter: Interpreter,
}

impl AgentRuntime {
    pub fn new() -> Self {
        let mut agents = HashMap::new();
        agents.insert("shell".to_string(), AgentInfo {
            name: "shell".to_string(),
            status: "ready".to_string(),
            last_output: String::new(),
        });
        agents.insert("web".to_string(), AgentInfo {
            name: "web".to_string(),
            status: "ready".to_string(),
            last_output: String::new(),
        });
        agents.insert("ai".to_string(), AgentInfo {
            name: "ai".to_string(),
            status: "ready".to_string(),
            last_output: String::new(),
        });
        Self {
            agents,
            interpreter: Interpreter::new(),
        }
    }

    pub fn list_agents(&self) -> Vec<AgentInfo> {
        self.agents.values().cloned().collect()
    }

    pub fn run_agent(&mut self, name: &str, input: &str) -> Result<String, String> {
        let result = match name {
            "shell" => {
                let output = std::process::Command::new("sh")
                    .args(["-c", input])
                    .output()
                    .map_err(|e| format!("Shell error: {}", e))?;
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            "web" => {
                let output = std::process::Command::new("curl")
                    .args(["-s", input])
                    .output()
                    .map_err(|e| format!("Curl error: {}", e))?;
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            "ai" => {
                let mut lexer = Lexer::new(input);
                let mut tokens = Vec::new();
                loop {
                    let token = lexer.next_token();
                    let is_eof = token.kind == tricore::token::TokenKind::EOF;
                    tokens.push(token);
                    if is_eof { break; }
                }
                let mut parser = Parser::new(tokens);
                match parser.parse() {
                    Ok(stmts) => {
                        let output = self.interpreter.run(&stmts);
                        output.join("\n")
                    }
                    Err(e) => format!("Lỗi: {}", e),
                }
            }
            _ => return Err(format!("Agent '{}' không tồn tại", name)),
        };

        if let Some(agent) = self.agents.get_mut(name) {
            agent.last_output = result.clone();
            agent.status = "completed".to_string();
        }

        Ok(result)
    }
}

pub type SharedRuntime = Arc<Mutex<AgentRuntime>>;

pub fn new_shared_runtime() -> SharedRuntime {
    Arc::new(Mutex::new(AgentRuntime::new()))
}
