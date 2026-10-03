use clap::Args;
use std::fs;
use std::env;

use crate::expand_env_vars;
use crate::config::{
    Session,
    Config,
};


/// Load a session from a predefined config
#[derive(Debug, Args)]
pub struct LoadSession {
    /// Name of the session from config
    pub name: String,

    /// Do not attach to the tmux server
    #[arg(short = 'a')]
    pub no_attach: bool,

    /// Initiate a nix shell
    #[arg(short = 'k')]
    pub nix_shell: bool,

    /// Rename the session based on the nix session
    #[arg(short = 'r')]
    pub nix_rename: bool,
}

pub fn load_session (config: Config, session: &LoadSession) ->  Session
{
    let mut s = config.sessions.get(&session.name).unwrap_or_else(|| {
        eprintln!("No session with that name in the config file");
        std::process::exit(1);
    }).clone();

    let path = expand_env_vars(&s.path);
    let directory = fs::canonicalize(path).unwrap_or_else(|_| {
        eprintln!("Invalid directory");
        std::process::exit(1);
    });

    env::set_current_dir(&directory).expect("Failed to change directory");

    if s.title == "" {
        s.title = directory
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
    }

    s.attach = !session.no_attach;
    s.clone()
}
