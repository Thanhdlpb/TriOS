use clap::{Parser, Subcommand};
use tripatch::backup::BackupEngine;
use tripatch::doctor::DoctorEngine;
use tripatch::registry::PluginRegistry;
use tripatch::transaction::{TransactionContext, TransactionEngine};

#[derive(Parser)]
#[command(name = "tripm")]
#[command(version = "0.2")]
#[command(about = "TriPatch Manager")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Patch {
        #[command(subcommand)]
        action: PatchCommand,
    },
    Doctor,
    Backup,
}

#[derive(Subcommand)]
enum PatchCommand {
    List,
    Info { name: String },
    Search { keyword: String },
    Install { name: String },
}

fn main() {
    let cli = Cli::parse();

    let registry = PluginRegistry::new("tripatch/plugins");

    match cli.command {
        Some(Commands::Doctor) => {
            doctor();
        }

        Some(Commands::Backup) => {
            backup();
        }

        Some(Commands::Patch { action }) => match action {
            PatchCommand::List => {
                println!("Available patches:");

                for p in registry.list() {
                    println!("{} {}", p.manifest.name, p.manifest.version);
                }

                println!("Total: {}", registry.count());
            }

            PatchCommand::Info { name } => match registry.find(&name) {
                Some(p) => {
                    println!("Name: {}", p.manifest.name);
                    println!("Version: {}", p.manifest.version);
                    println!("Description: {}", p.manifest.description);
                }
                None => println!("Plugin not found"),
            },

            PatchCommand::Search { keyword } => {
                for p in registry.list() {
                    if p.manifest.name.contains(&keyword) {
                        println!("{} {}", p.manifest.name, p.manifest.version);
                    }
                }
            }

            PatchCommand::Install { name } => {
                let ctx = TransactionContext::new(name);

                match TransactionEngine::execute(&ctx) {
                    Ok(_) => println!("Patch installed successfully."),
                    Err(e) => {
                        eprintln!("Transaction failed: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        },

        None => {
            println!("TriPatch Manager");
            println!("Use:");
            println!("  tripm patch list");
            println!("  tripm patch info usb");
            println!("  tripm patch search usb");
            println!("  tripm patch install usb");
            println!("  tripm doctor");
            println!("  tripm backup");
        }
    }
}

fn doctor() {
    println!();
    println!("========== TriOS Doctor ==========");

    let engine = DoctorEngine::new();

    let report = engine.run();

    for r in &report.results {
        println!("[{:?}] {} - {}", r.status, r.name, r.message);
    }

    println!();

    println!("Health Score: {}", report.health_score());

    println!("==================================");
}

fn backup() {
    println!();
    println!("========== TriOS Backup ==========");

    match BackupEngine::backup_configuration() {
        Ok(path) => {
            println!("Backup completed.");
            println!("Saved to:");
            println!("{}", path.display());
        }
        Err(err) => {
            println!("Backup failed:");
            println!("{}", err);
        }
    }

    println!("==================================");
}
