use crate::agents::agent_trait::{Agent, AgentResult};
use std::collections::HashMap;
use tricore::lexer::Lexer;
use tricore::parser::Parser;
use tricore::interpreter::Interpreter;

pub struct AIAgent {
    interpreter: Interpreter,
}

impl AIAgent {
    pub fn new() -> Self {
        Self { interpreter: Interpreter::new() }
    }
}

impl Agent for AIAgent {
    fn name(&self) -> &str { "ai" }

    fn run(&mut self, input: &str) -> AgentResult {
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
                let mut data = HashMap::new();
                data.insert("output".to_string(), output.join("\n"));
                AgentResult { success: true, message: "OK".to_string(), data }
            }
            Err(e) => AgentResult { success: false, message: e, data: HashMap::new() },
        }
    }
}
