use std::vec::Vec;

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

    let body: Vec<Version> = get_mod_versions(&args.mod_name).await;

    // println!("{} versions available.", body.len());
    // println!("Print how many?");
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();
    // let num_versions: usize = input.trim().parse().unwrap_or(0);

    // for i in 0..num_versions {
    //     print_version_info(&body[i]);
    // }

    let mut game_versions: Vec<Vec<String>> = Vec::new();

    for version in body {
        game_versions.push(version.game_versions);
    }
    println!("Available versions:");
    for versions in game_versions {
        println!("  - {}", versions.join(", "));
    }
}

async fn get_mod_versions(mod_name: &str) -> Vec<Version> {
    let url: String = format!("https://api.modrinth.com/v2/project/{}/version", mod_name);
    let response: reqwest::Response = reqwest::get(&url).await.unwrap();

    return response.json::<Vec<Version>>().await.unwrap();
}

fn print_version_info(version: &Version) {
    println!("-----------------------------");
    println!("ID: {}", version.id);
    println!("Name: {}", version.name);
    println!("Version Number: {}", version.version_number);
    println!("Version Type: {}", version.version_type);
    println!("Loaders: {:?}", version.loaders);
    println!("Game Versions: {:?}", version.game_versions);
    println!("Files:");
    for file in &version.files {
        println!("  - URL: {}", file.url);
        println!("    Filename: {}", file.filename);
        println!("    Primary: {}", file.primary);
    }
}
