use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::fs;

pub fn hoc_tu_tep(interpreter: &mut Interpreter, filename: &str) -> Result<usize, String> {
    let content =
        fs::read_to_string(filename).map_err(|e| format!("Không đọc được file: {}", e))?;
    let sentences: Vec<&str> = content
        .split(|c| c == '.' || c == '?' || c == '!')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let mut count = 0;
    for sentence in sentences {
        let input = format!("{}.", sentence);
        let mut lexer = Lexer::new(&input);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token();
            let is_eof = token.kind == crate::token::TokenKind::EOF;
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        let mut parser = Parser::new(tokens);
        if let Ok(statements) = parser.parse() {
            interpreter.run(&statements);
            count += 1;
        }
    }
    Ok(count)
}
