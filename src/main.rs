// main.rs
// created on 5/24/26

use crate::models::Version;
use clap::Parser;
use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;
mod models;

#[derive(Parser)]
struct Cli {
    mod_name: String,
}

#[tokio::main]
async fn main() {
    let args: Cli = Cli::parse();
    let body: Vec<Version> = get_mod_versions(&args.mod_name).await;

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
    println!("https://modrinth.com/project/{}", &args.mod_name);
    println!("Available versions for '{}':", &args.mod_name);
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

async fn get_mod_versions(mod_name: &str) -> Vec<Version> {
    let url: String = format!("https://api.modrinth.com/v2/project/{}/version", mod_name);
    let response: reqwest::Response = reqwest::get(&url).await.unwrap();

    return response.json::<Vec<Version>>().await.unwrap();
}
