mod agents {
    pub mod agent_trait {
        use std::collections::HashMap;

        #[derive(Debug, Clone)]
        pub struct AgentResult {
            pub success: bool,
            pub message: String,
            pub data: HashMap<String, String>,
        }

        pub trait Agent: Send + Sync {
            fn name(&self) -> &str;
            fn run(&mut self, input: &str) -> AgentResult;
        }
    }

    pub mod agent_manager {
        use super::agent_trait::{Agent, AgentResult};
        use std::collections::HashMap;

        pub struct AgentManager {
            agents: HashMap<String, Box<dyn Agent>>,
        }

        impl AgentManager {
            pub fn new() -> Self {
                Self {
                    agents: HashMap::new(),
                }
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
    }

    pub mod shell_agent {
        use super::agent_trait::{Agent, AgentResult};
        use std::collections::HashMap;
        use std::process::Command;

        pub struct ShellAgent;

        impl Agent for ShellAgent {
            fn name(&self) -> &str {
                "shell"
            }

            fn run(&mut self, cmd: &str) -> AgentResult {
                let output = Command::new("sh").args(["-c", cmd]).output();
                match output {
                    Ok(o) => {
                        let mut data = HashMap::new();
                        data.insert(
                            "stdout".to_string(),
                            String::from_utf8_lossy(&o.stdout).to_string(),
                        );
                        data.insert(
                            "stderr".to_string(),
                            String::from_utf8_lossy(&o.stderr).to_string(),
                        );
                        AgentResult {
                            success: true,
                            message: "OK".to_string(),
                            data,
                        }
                    }
                    Err(e) => AgentResult {
                        success: false,
                        message: format!("Shell error: {}", e),
                        data: HashMap::new(),
                    },
                }
            }
        }
    }

    pub mod web_agent {
        use super::agent_trait::{Agent, AgentResult};
        use std::collections::HashMap;
        use std::process::Command;

        pub struct WebAgent;

        impl Agent for WebAgent {
            fn name(&self) -> &str {
                "web"
            }

            fn run(&mut self, url: &str) -> AgentResult {
                let output = Command::new("curl").args(["-s", url]).output();
                match output {
                    Ok(o) => {
                        let mut data = HashMap::new();
                        data.insert(
                            "body".to_string(),
                            String::from_utf8_lossy(&o.stdout).to_string(),
                        );
                        AgentResult {
                            success: true,
                            message: "OK".to_string(),
                            data,
                        }
                    }
                    Err(e) => AgentResult {
                        success: false,
                        message: format!("curl error: {}", e),
                        data: HashMap::new(),
                    },
                }
            }
        }
    }

    pub mod ai_agent {
        use super::agent_trait::{Agent, AgentResult};
        use std::collections::HashMap;
        use tricore::interpreter::Interpreter;
        use tricore::lexer::Lexer;
        use tricore::parser::Parser;

        pub struct AIAgent {
            interpreter: Interpreter,
        }

        impl AIAgent {
            pub fn new() -> Self {
                Self {
                    interpreter: Interpreter::new(),
                }
            }
        }

        impl Agent for AIAgent {
            fn name(&self) -> &str {
                "ai"
            }

            fn run(&mut self, input: &str) -> AgentResult {
                let mut lexer = Lexer::new(input);
                let mut tokens = Vec::new();
                loop {
                    let token = lexer.next_token();
                    let is_eof = token.kind == tricore::token::TokenKind::EOF;
                    tokens.push(token);
                    if is_eof {
                        break;
                    }
                }
                let mut parser = Parser::new(tokens);
                match parser.parse() {
                    Ok(stmts) => {
                        let output = self.interpreter.run(&stmts);
                        let mut data = HashMap::new();
                        data.insert("output".to_string(), output.join("\n"));
                        AgentResult {
                            success: true,
                            message: "OK".to_string(),
                            data,
                        }
                    }
                    Err(e) => AgentResult {
                        success: false,
                        message: e,
                        data: HashMap::new(),
                    },
                }
            }
        }
    }
}

pub struct AgentRuntime {
    manager: agents::agent_manager::AgentManager,
}

impl AgentRuntime {
    pub fn new() -> Self {
        Self {
            manager: agents::agent_manager::AgentManager::new(),
        }
    }

    pub fn init(&mut self) {
        self.manager
            .register(Box::new(agents::shell_agent::ShellAgent));
        self.manager.register(Box::new(agents::web_agent::WebAgent));
        self.manager
            .register(Box::new(agents::ai_agent::AIAgent::new()));
    }

    pub fn run_agent(&mut self, name: &str, input: &str) -> String {
        match self.manager.run_agent(name, input) {
            Some(result) => {
                if result.success {
                    if let Some(out) = result
                        .data
                        .get("stdout")
                        .or_else(|| result.data.get("output"))
                        .or_else(|| result.data.get("body"))
                    {
                        out.clone()
                    } else {
                        result.message
                    }
                } else {
                    format!("❌ {}: {}", name, result.message)
                }
            }
            None => format!("❌ Agent '{}' không tồn tại", name),
        }
    }

    pub fn list_agents(&self) -> Vec<String> {
        self.manager.list_agents()
    }
}
