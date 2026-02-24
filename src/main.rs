use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Tauri { name: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Tauri { name } => {
            println!("Executando `cargo create-tauri-app {}`", name);
            run_tauri(&name);
        }
    }
}

fn run_tauri(project_name: &str) {
    let status = Command::new("cargo")
        .arg("create-tauri-app")
        .arg(project_name)
        .status()
        .expect("Falha ao executar cargo create-tauri-app");

    if status.success() {
        println!("Projeto Tauri criado com sucesso!");
    } else {
        eprintln!("Erro ao criar projeto Tauri");
    }
}
