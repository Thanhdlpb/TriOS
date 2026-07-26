use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

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

fn tao_thu_vien(ten: &str) {
    println!("✅ Đã tạo thư viện '{}' (tính năng đang được xây dựng)", ten);
}

fn tao_plugin(ten: &str) {
    println!("✅ Đã tạo plugin '{}' (tính năng đang được xây dựng)", ten);
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Tao { ung_dung, thu_vien, plugin } => {
            if let Some(name) = ung_dung {
                tao_ung_dung(&name);
            } else if let Some(name) = thu_vien {
                tao_thu_vien(&name);
            } else if let Some(name) = plugin {
                tao_plugin(&name);
            } else {
                println!("Dùng: tri tao --ung-dung <tên>");
            }
        }
    }
}
