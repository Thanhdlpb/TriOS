use actix_web::{web, App, HttpServer, HttpResponse, middleware};
use actix_files::Files;
use serde::{Deserialize, Serialize};
use tricore::lexer::Lexer;
use tricore::parser::Parser as TriParser;
use tricore::interpreter::Interpreter;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct RunRequest {
    code: String,
}

#[derive(Serialize)]
struct RunResponse {
    output: Vec<String>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct SaveRequest {
    file_name: String,
    code: String,
}

#[derive(Serialize)]
struct SaveResponse {
    message: String,
}

async fn run_code(req: web::Json<RunRequest>) -> HttpResponse {
    let mut lexer = Lexer::new(&req.code);
    let mut tokens = Vec::new();
    
    // Lex
    loop {
        let token = lexer.next_token();
        let is_eof = token.kind == tricore::token::TokenKind::EOF;
        tokens.push(token);
        if is_eof { break; }
    }
    
    // Parse
    let mut parser = TriParser::new(tokens);
    match parser.parse_chuong_trinh() {
        Ok(statements) => {
            let mut interpreter = Interpreter::new();
            let output = interpreter.run(&statements);
            HttpResponse::Ok().json(RunResponse {
                output,
                error: None,
            })
        }
        Err(e) => {
            HttpResponse::Ok().json(RunResponse {
                output: vec![],
                error: Some(e),
            })
        }
    }
}

async fn save_code(req: web::Json<SaveRequest>) -> HttpResponse {
    let dir_path = Path::new("du_lieu");
    fs::create_dir_all(dir_path).ok();
    let file_path = dir_path.join(&req.file_name);
    
    match fs::write(&file_path, &req.code) {
        Ok(_) => HttpResponse::Ok().json(SaveResponse {
            message: format!("✅ Đã lưu vào {}", file_path.display()),
        }),
        Err(e) => HttpResponse::Ok().json(SaveResponse {
            message: format!("❌ Lỗi: {}", e),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🌱 Tri Studio đang chạy tại http://localhost:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .route("/run", web::post().to(run_code))
                    .route("/save", web::post().to(save_code))
            )
            .service(Files::new("/static", "tristudio/static").show_files_listing())
            .route("/", web::get().to(|| async {
                HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(include_str!("../static/index.html"))
            }))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
