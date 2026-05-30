// cli.rs
// created on 5/29/26

use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "mcli")]
#[command(about = "Modrinth CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get available versions for a mod
    Versions(VersionsArgs),
    /// Download a mod with the specified paremeters
    Download(DownloadArgs),
}

#[derive(Args)]
pub struct VersionsArgs {
    /// The mod name (slug)
    pub mod_name: String,
}

#[derive(Args)]
pub struct DownloadArgs {
    /// The mod name (slug)
    pub mod_name: String,
    /// The mod loader (e.g. fabric)
    #[arg(default_value_t = Loader::Fabric)]
    pub loader: Loader,
    /// alpha, beta, or release
    #[arg(default_value_t = Channel::Release)]
    pub channel: Channel,
}

#[derive(Clone, ValueEnum, PartialEq, Eq)]
#[clap(rename_all = "lowercase")]
pub enum Loader {
    Fabric,
    Forge,
    NeoForge,
    Quilt, // There are more that I'm not including yet
}
impl std::fmt::Display for Loader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Loader::Fabric => write!(f, "fabric"),
            Loader::Forge => write!(f, "forge"),
            Loader::NeoForge => write!(f, "neoforge"),
            Loader::Quilt => write!(f, "quilt"),
        }
    }
}
#[derive(Clone, Debug, Deserialize, clap::ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[clap(rename_all = "lowercase")]
pub enum Channel {
    Release,
    Beta,
    Alpha,
}
impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Channel::Release => write!(f, "release"),
            Channel::Beta => write!(f, "beta"),
            Channel::Alpha => write!(f, "alpha"),
        }
    }
}
