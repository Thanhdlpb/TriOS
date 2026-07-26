use crate::bytecode::*;
use std::collections::HashMap;

pub struct VM {
    stack: Vec<f64>,
    variables: HashMap<String, f64>,
    ip: usize,
    running: bool,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            variables: HashMap::new(),
            ip: 0,
            running: false,
        }
    }

    pub fn run(&mut self, program: &Bytecode) {
        self.running = true;
        while self.running && self.ip < program.instructions.len() {
            let inst = &program.instructions[self.ip];
            self.execute(inst);
            self.ip += 1;
        }
    }

    fn execute(&mut self, inst: &OpCode) {
        match inst {
            OpCode::Push(val) => self.stack.push(*val),
            OpCode::Pop => { self.stack.pop(); }
            OpCode::Add => {
                let b = self.stack.pop().unwrap_or(0.0);
                let a = self.stack.pop().unwrap_or(0.0);
                self.stack.push(a + b);
            }
            OpCode::Sub => {
                let b = self.stack.pop().unwrap_or(0.0);
                let a = self.stack.pop().unwrap_or(0.0);
                self.stack.push(a - b);
            }
            OpCode::Mul => {
                let b = self.stack.pop().unwrap_or(0.0);
                let a = self.stack.pop().unwrap_or(0.0);
                self.stack.push(a * b);
            }
            OpCode::Div => {
                let b = self.stack.pop().unwrap_or(1.0);
                let a = self.stack.pop().unwrap_or(0.0);
                self.stack.push(a / b);
            }
            OpCode::Load(name) => {
                if let Some(val) = self.variables.get(name) {
                    self.stack.push(*val);
                }
            }
            OpCode::Store(name) => {
                if let Some(val) = self.stack.pop() {
                    self.variables.insert(name.clone(), val);
                }
            }
            OpCode::Print => {
                if let Some(val) = self.stack.pop() {
                    println!("{}", val);
                }
            }
            OpCode::Halt => self.running = false,
            _ => {}
        }
    }
}
