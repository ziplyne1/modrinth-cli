use crate::models::Version;
use clap::Parser;
mod models;

#[derive(Parser)]
struct Cli {
    mod_name: String,
}

#[tokio::main]
async fn main() {
    let args: Cli = Cli::parse();

    let url: String = format!(
        "https://api.modrinth.com/v2/project/{}/version",
        args.mod_name
    );
    let response: reqwest::Response = reqwest::get(&url).await.unwrap();
    let body: Vec<Version> = response.json::<Vec<Version>>().await.unwrap();

    println!("ID: {}", body[0].id);
    println!("Name: {}", body[0].name);
    println!("Version Number: {}", body[0].version_number);
    println!("Version Type: {}", body[0].version_type);
    println!("Loaders: {:?}", body[0].loaders);
    println!("Game Versions: {:?}", body[0].game_versions);
    println!("Files:");
    for file in &body[0].files {
        println!("  - URL: {}", file.url);
        println!("    Filename: {}", file.filename);
        println!("    Primary: {}", file.primary);
    }
}