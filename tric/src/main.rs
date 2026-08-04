use std::env;
use std::fs;
use tricore::interpreter::Interpreter;
use tricore::lexer::Lexer;
use tricore::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Sử dụng: tric <file.tri>");
        return;
    }
    let source = fs::read_to_string(&args[1]).expect("Không thể đọc file");
    let mut lexer = Lexer::new(&source);
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
            let mut interpreter = Interpreter::new();
            let output = interpreter.run(&stmts);
            for line in output {
                println!("{}", line);
            }
        }
        Err(e) => eprintln!("Lỗi: {}", e),
    }
}
