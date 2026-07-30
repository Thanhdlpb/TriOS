use actix_web::{web, App, HttpServer, HttpResponse};
use actix_files::Files;
use serde::{Deserialize, Serialize};
use tricore::lexer::Lexer;
use tricore::parser::Parser as TriParser;
use tricore::interpreter::Interpreter;
use std::fs;
use std::path::Path;
use agent_runtime::SharedRuntime;

mod agent_runtime;

#[derive(Deserialize)]
struct RunRequest { code: String }

#[derive(Serialize)]
struct RunResponse { output: Vec<String>, error: Option<String> }

#[derive(Deserialize)]
struct SaveRequest { file_name: String, code: String }

#[derive(Deserialize)]
struct AgentRequest { agent: String, input: String }

#[derive(Serialize)]
struct AgentResponse { output: String, error: Option<String> }

#[derive(Serialize)]
struct AgentsListResponse { agents: Vec<agent_runtime::AgentInfo> }

async fn run_code(req: web::Json<RunRequest>) -> HttpResponse {
    println!("📝 [API /run] Nhận code: {} ký tự", req.code.len());
    let mut lexer = Lexer::new(&req.code);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        let is_eof = token.kind == tricore::token::TokenKind::EOF;
        tokens.push(token);
        if is_eof { break; }
    }
    let mut parser = TriParser::new(tokens);
    match parser.parse() {
        Ok(statements) => {
            let mut interpreter = Interpreter::new();
            let output = interpreter.run(&statements);
            println!("✅ [API /run] Thành công, {} dòng output", output.len());
            HttpResponse::Ok().json(RunResponse { output, error: None })
        }
        Err(e) => {
            println!("❌ [API /run] Lỗi parse: {}", e);
            HttpResponse::Ok().json(RunResponse { output: vec![], error: Some(e) })
        }
    }
}

async fn save_code(req: web::Json<SaveRequest>) -> HttpResponse {
    println!("💾 [API /save] Lưu file: {}", req.file_name);
    let dir_path = Path::new("du_lieu");
    fs::create_dir_all(dir_path).ok();
    let file_path = dir_path.join(&req.file_name);
    match fs::write(&file_path, &req.code) {
        Ok(_) => {
            println!("✅ [API /save] Đã lưu vào {}", file_path.display());
            HttpResponse::Ok().json(serde_json::json!({"message": format!("Đã lưu vào {}", file_path.display())}))
        }
        Err(e) => {
            println!("❌ [API /save] Lỗi: {}", e);
            HttpResponse::Ok().json(serde_json::json!({"message": format!("Lỗi: {}", e)}))
        }
    }
}

async fn run_agent(
    req: web::Json<AgentRequest>,
    runtime: web::Data<SharedRuntime>,
) -> HttpResponse {
    println!("🤖 [API /agent/run] Agent: {}, Input: {}", req.agent, req.input);
    let mut rt = runtime.lock();
    match rt.run_agent(&req.agent, &req.input) {
        Ok(output) => {
            println!("✅ [API /agent/run] Output: {} ký tự", output.len());
            HttpResponse::Ok().json(AgentResponse { output, error: None })
        }
        Err(e) => {
            println!("❌ [API /agent/run] Lỗi: {}", e);
            HttpResponse::Ok().json(AgentResponse { output: String::new(), error: Some(e) })
        }
    }
}

async fn list_agents(runtime: web::Data<SharedRuntime>) -> HttpResponse {
    println!("📋 [API /agent/list] Liệt kê agents");
    let rt = runtime.lock();
    let agents = rt.list_agents();
    HttpResponse::Ok().json(AgentsListResponse { agents })
}

async fn dashboard() -> HttpResponse {
    println!("🌐 [WEB] Dashboard được truy cập");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/dashboard.html"))
}

async fn index() -> HttpResponse {
    println!("🌐 [WEB] Studio được truy cập");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🌱 TriOS Studio & Dashboard đang khởi động...");
    println!("📡 Địa chỉ: http://localhost:8080");
    println!("📊 Dashboard: http://localhost:8080/dashboard");
    
    let shared_runtime = agent_runtime::new_shared_runtime();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_runtime.clone()))
            .service(
                web::scope("/api")
                    .route("/run", web::post().to(run_code))
                    .route("/save", web::post().to(save_code))
                    .route("/agent/run", web::post().to(run_agent))
                    .route("/agent/list", web::get().to(list_agents))
            )
            .service(Files::new("/static", "tristudio/static").show_files_listing())
            .route("/", web::get().to(index))
            .route("/dashboard", web::get().to(dashboard))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
