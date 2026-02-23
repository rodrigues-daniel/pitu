use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Arquivo de configuração TOML
    #[arg(long)]
    config: Option<String>,
}

#[derive(Deserialize)]
struct Config {
    username: Option<String>,
    verbose: Option<bool>,
    max_connections: Option<u32>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    // Lê config.toml se existir
    let config_path = args.config.unwrap_or_else(|| "config.toml".to_string());
    let config = if Path::new(&config_path).exists() {
        let contents = fs::read_to_string(&config_path).expect("Falha ao ler o arquivo");
        toml::from_str::<Config>(&contents).expect("Falha ao parsear TOML")
    } else {
        Config {
            username: None,
            verbose: Some(false),
            max_connections: None,
        }
    };

    if config.verbose.unwrap_or(false) {
        println!("Modo verbose ativado");
    }

    let username = config
        .username
        .unwrap_or_else(|| "Usuário padrão".to_string());
    println!("Usuário: {}", username);

    // Exemplo de integração com ferramenta CLI externa (ls)
    let output = Command::new("ls")
        .arg("-l")
        .output()
        .expect("Falha ao executar comando externo");
    println!(
        "Saída do comando ls:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );

    // Exemplo de integração com API externa usando reqwest
    if let Ok(resp) = reqwest::get("https://api.github.com/repos/rust-lang/rust")
        .await
        .and_then(|r| r.json::<serde_json::Value>().await)
    {
        println!("Descrição do repositório Rust: {}", resp["description"]);
    }
}
