mod bytecode;
mod loader;
mod executor;

use loader::BytecodeProgram;
use executor::VM;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("TriVM - Máy ảo TriOS");
        println!("Sử dụng: trivm <file.tbc>");
        return;
    }
    
    let filename = &args[1];
    match BytecodeProgram::from_file(filename) {
        Ok(program) => {
            if let Err(e) = program.verify() {
                eprintln!("Lỗi xác thực bytecode: {}", e);
                return;
            }
            let mut vm = VM::new();
            match vm.run(&program) {
                Ok(output) => {
                    for line in output {
                        println!("{}", line);
                    }
                }
                Err(e) => eprintln!("Lỗi thực thi: {}", e),
            }
        }
        Err(e) => eprintln!("Lỗi đọc file: {}", e),
    }
}
