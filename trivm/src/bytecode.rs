#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    Push(f64),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Load(String),
    Store(String),
    Call(String),
    Ret,
    Jmp(usize),
    Jz(usize),
    Print,
    Halt,
}

#[derive(Debug, Clone)]
pub struct Bytecode {
    pub instructions: Vec<OpCode>,
}
