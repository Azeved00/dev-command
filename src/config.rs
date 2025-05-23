use serde::Deserialize;
use std::collections::HashMap;
use std::vec::Vec;
use std::{
    fs,
    path::PathBuf,
};

#[derive(Deserialize, Clone, Debug)]
pub struct Session{
    #[serde(default = "default_windows")]
    pub windows: Vec<Window>,
    pub path: String,
    #[serde(default = "default_title")]
    pub title: String,
}
pub fn default_windows() -> Vec<Window> { 
    vec![
        Window {
            title: "".to_string(), 
            nix_shell: "".to_string(),
            nix_rename: false,
        }
    ]
}
fn default_title() -> String { "".to_string() }
fn default_nix_rename() -> bool { false }
fn default_nix_shell_name() -> String { "".to_string() }

#[derive(Deserialize)]
pub struct Config{
    pub sessions: HashMap<String, Session>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Window{
    #[serde(default = "default_title")]
    pub title: String,
    #[serde(default = "default_nix_shell_name")]
    pub nix_shell: String,
    #[serde(default = "default_nix_rename")]
    pub nix_rename: bool,
}

impl Config {
    pub fn get_config(path:PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?; 
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
