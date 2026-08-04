use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use tricore::hoc_tap;
use tricore::interpreter::Interpreter;
use tricore::lexer::Lexer;
use tricore::parser::Parser as TriParser;

#[derive(Parser)]
#[command(name = "tri")]
#[command(about = "TriOS CLI – Công cụ khởi tạo và quản lý dự án")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Tao {
        #[arg(short, long)]
        ung_dung: Option<String>,
        #[arg(short, long)]
        thu_vien: Option<String>,
        #[arg(short, long)]
        plugin: Option<String>,
    },
    Chay {
        file: Option<String>,
    },
}

fn sao_chep_thu_muc(tu: &str, den: &str, ten: &str) -> std::io::Result<()> {
    fs::create_dir_all(den)?;
    for entry in fs::read_dir(tu)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let dest_path = Path::new(den).join(file_name);
        if path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            sao_chep_thu_muc(path.to_str().unwrap(), dest_path.to_str().unwrap(), ten)?;
        } else {
            let content = fs::read_to_string(&path)?;
            let replaced = content.replace("{{TEN}}", ten);
            fs::write(&dest_path, replaced)?;
        }
    }
    Ok(())
}

fn tao_ung_dung(ten: &str) {
    let mau_dir = "mau/ung_dung";
    let dich_dir = format!("ung_dung/{}", ten);
    sao_chep_thu_muc(mau_dir, &dich_dir, ten).unwrap();
    println!("✅ Đã tạo ứng dụng '{}' trong thư mục '{}'", ten, dich_dir);
}

fn chay_file(filename: &str) {
    let source = fs::read_to_string(filename).expect("Không thể đọc file");
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
    let mut parser = TriParser::new(tokens);
    match parser.parse() {
        Ok(statements) => {
            let mut interpreter = Interpreter::new();
            let output = interpreter.run(&statements);
            for line in output {
                println!("{}", line);
            }
        }
        Err(e) => eprintln!("Lỗi: {}", e),
    }
}

fn xu_ly_ai(interpreter: &mut Interpreter, input: &str) {
    let parts: Vec<&str> = input.splitn(3, ' ').collect();
    if parts.len() < 2 {
        println!("Sử dụng: ai huan_luyen <file> hoặc ai du_doan <input>");
        return;
    }
    match parts[0] {
        "huan_luyen" => {
            let count = hoc_tap::hoc_tu_tep(interpreter, parts[1]).unwrap_or(0);
            println!("✅ Đã học {} câu từ '{}'", count, parts[1]);
        }
        "du_doan" => {
            println!("🔮 Tính năng dự đoán đang được xây dựng.");
        }
        _ => println!("Lệnh AI không hợp lệ."),
    }
}

fn xu_ly_cau_lenh(interpreter: &mut Interpreter, input: &str) {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return;
    }

    if let Some(ai_input) = trimmed.strip_prefix("ai ") {
        xu_ly_ai(interpreter, ai_input);
        return;
    }

    if let Some(filepath) = trimmed.strip_prefix("lưu ") {
        match interpreter.luu(filepath.trim()) {
            Ok(()) => println!("✅ Đã lưu tri thức vào '{}'", filepath.trim()),
            Err(e) => eprintln!("Lỗi: {}", e),
        }
        return;
    }
    if let Some(filepath) = trimmed.strip_prefix("nạp ") {
        match interpreter.nap(filepath.trim()) {
            Ok(()) => println!("✅ Đã nạp tri thức từ '{}'", filepath.trim()),
            Err(e) => eprintln!("Lỗi: {}", e),
        }
        return;
    }
    if let Some(filepath) = trimmed.strip_prefix("học ") {
        match hoc_tap::hoc_tu_tep(interpreter, filepath.trim()) {
            Ok(count) => println!("✅ Đã học {} câu từ '{}'", count, filepath.trim()),
            Err(e) => eprintln!("Lỗi: {}", e),
        }
        return;
    }

    let processed = if !trimmed.ends_with('.') && !trimmed.ends_with('?') {
        format!("{}.", trimmed)
    } else {
        trimmed.to_string()
    };

    let mut lexer = Lexer::new(&processed);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        let is_eof = token.kind == tricore::token::TokenKind::EOF;
        tokens.push(token);
        if is_eof {
            break;
        }
    }
    let mut parser = TriParser::new(tokens);
    match parser.parse() {
        Ok(statements) => {
            let output = interpreter.run(&statements);
            for line in output {
                println!("{}", line);
            }
        }
        Err(e) => eprintln!("❌ Lỗi: {}", e),
    }
}

fn repl() {
    println!("🌱 TriOS REPL (gõ 'thoát' để dừng)");
    println!("   Lệnh: lưu <file>, nạp <file>, học <file>, ai huan_luyen <file>");
    let mut interpreter = Interpreter::new();
    let mut input_buffer = String::new();
    loop {
        print!("tri> ");
        io::stdout().flush().unwrap();
        input_buffer.clear();
        if io::stdin().read_line(&mut input_buffer).is_err() {
            break;
        }
        let trimmed = input_buffer.trim();
        if trimmed == "thoát" || trimmed == "exit" || trimmed == "quit" {
            break;
        }
        xu_ly_cau_lenh(&mut interpreter, trimmed);
    }
    println!("Đã thoát REPL.");
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Tao {
            ung_dung,
            thu_vien,
            plugin,
        } => {
            if let Some(name) = ung_dung {
                tao_ung_dung(&name);
            } else if let Some(name) = thu_vien {
                println!("✅ Đã tạo thư viện '{}'", name);
            } else if let Some(name) = plugin {
                println!("✅ Đã tạo plugin '{}'", name);
            } else {
                println!("Dùng: tri tao --ung-dung <tên>");
            }
        }
        Commands::Chay { file } => match file {
            Some(filename) => chay_file(&filename),
            None => repl(),
        },
    }
}
