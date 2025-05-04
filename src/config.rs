use serde::Deserialize;
use std::collections::HashMap;
use std::{
    fs,
    path::PathBuf,
};

#[derive(Deserialize, Clone, Debug)]
pub struct Session{
    #[serde(default = "default_num_windows")]
    pub windows: i64,
    #[serde(default = "default_nix_shell")]
    pub nix_shell: String,
    pub path: String,
    #[serde(default = "default_title")]
    pub title: String,
    #[serde(default = "default_attach")]
    pub attach: bool,
    #[serde(default = "default_nix_rename")]
    pub nix_rename: bool,
}
fn default_num_windows() -> i64{ 1 }
fn default_nix_shell() -> String { "default".to_string() }
fn default_title() -> String { "".to_string() }
fn default_attach() -> bool { true }
fn default_nix_rename() -> bool { false }

#[derive(Deserialize)]
pub struct Config{
    pub sessions: HashMap<String, Session>,
}

impl Config {
    pub fn get_config(path:PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path.as_ref())?; 
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
