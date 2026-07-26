use std::fs;
use std::env;
use tricore::lexer::Lexer;
use tricore::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Sử dụng: tric <file.tri>");
        return;
    }
    let filename = &args[1];
    let source = fs::read_to_string(filename).expect("Không thể đọc file");
    let mut lexer = Lexer::new(&source);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        let is_eof = token.kind == tricore::token::TokenKind::EOF;
        tokens.push(token);
        if is_eof { break; }
    }
    let mut parser = Parser::new(tokens);
    match parser.parse_chuong_trinh() {
        Ok(ast) => {
            for stmt in &ast {
                println!("{}", stmt);
            }
        }
        Err(e) => eprintln!("Lỗi phân tích cú pháp: {}", e),
    }
}
