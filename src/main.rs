// main.rs
// created on 5/24/26

mod cli;
mod models;

use cli::{Channel, Cli, Commands, Loader};
use models::Version;

use clap::Parser;

use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Versions(args) => print_available_versions(&args.mod_name).await,
        Commands::Download(args) => {
            download_latest(&args.mod_name, &args.loader, &args.channel).await
        }
    }
}

async fn get_mod_versions(mod_name: &str) -> Vec<Version> {
    let url: String = format!("https://api.modrinth.com/v2/project/{}/version", mod_name);
    let response: reqwest::Response = reqwest::get(&url).await.unwrap();

    response.json::<Vec<Version>>().await.unwrap()
}

async fn print_available_versions(mod_name: &str) {
    let body: Vec<Version> = get_mod_versions(mod_name).await;

    let mut available_versions: HashMap<String, HashSet<String>> = HashMap::new();

    // This loop *copies* all the game_versions into `game_versions`
    for version in &body {
        for game_version in &version.game_versions {
            if !available_versions.contains_key(game_version) {
                available_versions.insert(game_version.clone(), HashSet::new());
            }

            available_versions
                .get_mut(game_version)
                .unwrap()
                .insert(version.id.clone());
        }
    }

    let mut keys = available_versions.keys().cloned().collect::<Vec<String>>();
    keys.sort();
    keys.reverse();

    let longest_key_length = keys
        .iter()
        .max_by_key(|s| s.len())
        .unwrap_or(&String::new())
        .len();

    println!();
    println!("https://modrinth.com/project/{}", &mod_name);
    println!("Available versions for '{}':", &mod_name);
    for key in keys {
        println!(
            "- {}{} ({})",
            key,
            " ".repeat(longest_key_length - key.len()),
            available_versions
                .get(&key)
                .unwrap_or(&HashSet::new())
                .len()
        );
    }
}

async fn download_latest(mod_name: &str, loader: &Loader, channel: &Channel) {
    let versions = get_mod_versions(mod_name).await;
    let mut candidate_version = &versions[0];

    for version in &versions {
        if !(&version.channel == channel) {
            continue;
        }
        if !version.loaders.contains(&loader.to_string()) {
            continue;
        }
        if version.version_number <= candidate_version.version_number {
            continue;
        }

        candidate_version = version;
    }

    println!();
    println!("Selected version: {}", candidate_version.name);
    println!("Channel: {}", candidate_version.channel);
}
