use crate::bytecode::OpCode;
use crate::loader::BytecodeProgram;
use std::collections::HashMap;

pub struct VM {
    stack: Vec<f64>,
    variables: HashMap<String, f64>,
    pc: usize,
    running: bool,
    output: Vec<String>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            variables: HashMap::new(),
            pc: 0,
            running: false,
            output: Vec::new(),
        }
    }

    pub fn run(&mut self, program: &BytecodeProgram) -> Result<Vec<String>, String> {
        self.pc = program.entry_point;
        self.running = true;
        self.output.clear();

        while self.running && self.pc < program.instructions.len() {
            let inst = program.instructions[self.pc].clone();
            self.execute(&inst)?;
        }

        Ok(self.output.clone())
    }

    fn execute(&mut self, inst: &OpCode) -> Result<(), String> {
        match inst {
            OpCode::Push(val) => {
                self.stack.push(*val);
                self.pc += 1;
            }
            OpCode::Pop => {
                self.stack.pop();
                self.pc += 1;
            }
            OpCode::Dup => {
                if let Some(&top) = self.stack.last() {
                    self.stack.push(top);
                }
                self.pc += 1;
            }
            OpCode::Swap => {
                let len = self.stack.len();
                if len >= 2 {
                    self.stack.swap(len - 1, len - 2);
                }
                self.pc += 1;
            }
            OpCode::Load(var) => {
                let val = self.variables.get(var).copied().unwrap_or(0.0);
                self.stack.push(val);
                self.pc += 1;
            }
            OpCode::Store(var) => {
                if let Some(val) = self.stack.pop() {
                    self.variables.insert(var.clone(), val);
                }
                self.pc += 1;
            }
            OpCode::Jmp(addr) => {
                self.pc = *addr;
            }
            OpCode::Jz(addr) => {
                if self.stack.pop().unwrap_or(0.0) == 0.0 {
                    self.pc = *addr;
                } else {
                    self.pc += 1;
                }
            }
            OpCode::Jnz(addr) => {
                if self.stack.pop().unwrap_or(0.0) != 0.0 {
                    self.pc = *addr;
                } else {
                    self.pc += 1;
                }
            }
            OpCode::Call(addr) => {
                self.stack.push(self.pc as f64 + 1.0);
                self.pc = *addr;
            }
            OpCode::Ret => {
                if let Some(addr) = self.stack.pop() {
                    self.pc = addr as usize;
                } else {
                    self.running = false;
                }
            }
            OpCode::Add => self.binary_op(|a, b| a + b),
            OpCode::Sub => self.binary_op(|a, b| a - b),
            OpCode::Mul => self.binary_op(|a, b| a * b),
            OpCode::Div => self.binary_op(|a, b| a / b),
            OpCode::Mod => self.binary_op(|a, b| a % b),
            OpCode::Eq => self.compare_op(|a, b| a == b),
            OpCode::Neq => self.compare_op(|a, b| a != b),
            OpCode::Gt => self.compare_op(|a, b| a > b),
            OpCode::Lt => self.compare_op(|a, b| a < b),
            OpCode::Gte => self.compare_op(|a, b| a >= b),
            OpCode::Lte => self.compare_op(|a, b| a <= b),
            OpCode::And => self.logic_op(|a, b| a != 0.0 && b != 0.0),
            OpCode::Or => self.logic_op(|a, b| a != 0.0 || b != 0.0),
            OpCode::Not => {
                if let Some(val) = self.stack.pop() {
                    self.stack.push(if val == 0.0 { 1.0 } else { 0.0 });
                }
                self.pc += 1;
            }
            OpCode::Print => {
                if let Some(val) = self.stack.pop() {
                    self.output.push(val.to_string());
                }
                self.pc += 1;
            }
            OpCode::PrintLn => {
                if let Some(val) = self.stack.pop() {
                    self.output.push(format!("{}", val));
                }
                self.pc += 1;
            }
            OpCode::Input => {
                // Tạm thời chưa hỗ trợ input
                self.stack.push(0.0);
                self.pc += 1;
            }
            OpCode::AssertFact(sub, pred, obj) => {
                self.output.push(format!("Fact: {} {} {}", sub, pred, obj));
                self.pc += 1;
            }
            OpCode::QueryFact(sub, pred, obj) => {
                self.output
                    .push(format!("Query: {} {} {} ?", sub, pred, obj));
                self.pc += 1;
            }
            OpCode::Halt => {
                self.running = false;
            }
            OpCode::Nop => {
                self.pc += 1;
            }
        }
        Ok(())
    }

    fn binary_op<F>(&mut self, op: F)
    where
        F: Fn(f64, f64) -> f64,
    {
        let b = self.stack.pop().unwrap_or(0.0);
        let a = self.stack.pop().unwrap_or(0.0);
        self.stack.push(op(a, b));
        self.pc += 1;
    }

    fn compare_op<F>(&mut self, op: F)
    where
        F: Fn(f64, f64) -> bool,
    {
        let b = self.stack.pop().unwrap_or(0.0);
        let a = self.stack.pop().unwrap_or(0.0);
        self.stack.push(if op(a, b) { 1.0 } else { 0.0 });
        self.pc += 1;
    }

    fn logic_op<F>(&mut self, op: F)
    where
        F: Fn(f64, f64) -> bool,
    {
        let b = self.stack.pop().unwrap_or(0.0);
        let a = self.stack.pop().unwrap_or(0.0);
        self.stack.push(if op(a, b) { 1.0 } else { 0.0 });
        self.pc += 1;
    }
}
