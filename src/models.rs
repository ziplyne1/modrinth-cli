// models.rs
// created on 5/25/26

use crate::cli::Channel;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Version {
    pub id: String,
    pub name: String,
    pub version_number: String,
    #[serde(rename = "version_type")]
    pub channel: Channel,
    pub loaders: Vec<String>,
    pub game_versions: Vec<String>,
    // pub files: Vec<VersionFile>,
}

// #[derive(Deserialize)]
// pub struct VersionFile {
//     pub url: String,
//     pub filename: String,
//     pub primary: bool,
// }
