use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "tripm")]
#[command(about = "TriOS Package Manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Cài đặt gói
    #[command(name = "cai")]
    Cai {
        /// Tên gói cần cài
        ten: String,
    },
    /// Gỡ bỏ gói
    #[command(name = "xoa")]
    Xoa {
        /// Tên gói cần gỡ
        ten: String,
    },
    /// Tìm kiếm gói
    #[command(name = "tim")]
    Tim {
        /// Từ khóa tìm kiếm
        tu_khoa: String,
    },
    /// Cập nhật tất cả gói
    #[command(name = "cap_nhat")]
    CapNhat,
    /// Khởi tạo gói mới
    #[command(name = "khoi_tao")]
    KhoiTao {
        /// Tên gói
        ten: String,
    },
}

fn thu_muc_packages() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".trios/packages")
}

fn cai(ten: &str) {
    let thu_muc = thu_muc_packages();
    fs::create_dir_all(&thu_muc).unwrap();
    let duong_dan = thu_muc.join(ten);
    if duong_dan.exists() {
        println!("⚠️ Gói '{}' đã được cài đặt.", ten);
    } else {
        fs::create_dir(&duong_dan).unwrap();
        println!("✅ Đã cài đặt gói '{}'", ten);
    }
}

fn xoa(ten: &str) {
    let thu_muc = thu_muc_packages();
    let duong_dan = thu_muc.join(ten);
    if duong_dan.exists() {
        fs::remove_dir_all(&duong_dan).unwrap();
        println!("✅ Đã gỡ bỏ gói '{}'", ten);
    } else {
        println!("❌ Gói '{}' chưa được cài đặt.", ten);
    }
}

fn tim(tu_khoa: &str) {
    println!("🔍 Tìm kiếm gói với từ khóa '{}'...", tu_khoa);
    println!("   (chức năng đang được xây dựng)");
}

fn cap_nhat() {
    let thu_muc = thu_muc_packages();
    if let Ok(entries) = fs::read_dir(&thu_muc) {
        let mut count = 0;
        for entry in entries.flatten() {
            println!("🔄 Đang cập nhật '{}'...", entry.file_name().to_string_lossy());
            count += 1;
        }
        println!("✅ Đã cập nhật {} gói", count);
    } else {
        println!("📦 Chưa có gói nào được cài đặt.");
    }
}

fn khoi_tao(ten: &str) {
    fs::create_dir_all(ten).unwrap();
    let metadata = format!(
        r#"{{
  "ten": "{}",
  "phien_ban": "0.1.0",
  "mo_ta": "",
  "tac_gia": "",
  "phu_thuoc": []
}}
"#,
        ten
    );
    fs::write(format!("{}/package.trip", ten), metadata).unwrap();
    println!("✅ Đã khởi tạo gói '{}'", ten);
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Cai { ten } => cai(&ten),
        Commands::Xoa { ten } => xoa(&ten),
        Commands::Tim { tu_khoa } => tim(&tu_khoa),
        Commands::CapNhat => cap_nhat(),
        Commands::KhoiTao { ten } => khoi_tao(&ten),
    }
}
