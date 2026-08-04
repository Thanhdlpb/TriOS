use crate::bytecode::OpCode;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct BytecodeProgram {
    pub instructions: Vec<OpCode>,
    pub entry_point: usize,
}

impl BytecodeProgram {
    pub fn from_file(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    }

    pub fn verify(&self) -> Result<(), String> {
        for (i, inst) in self.instructions.iter().enumerate() {
            match inst {
                OpCode::Jmp(addr) | OpCode::Jz(addr) | OpCode::Jnz(addr) => {
                    if *addr >= self.instructions.len() {
                        return Err(format!(
                            "Địa chỉ {} không hợp lệ tại instruction {}",
                            addr, i
                        ));
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
