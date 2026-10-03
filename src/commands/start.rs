use clap::Args;
use std::path::PathBuf;
use std::fs;
use std::env;

use crate::config::{
    Session,
    Config,
    default_windows,
};

/// Create a new tmux session from a directory
#[derive(Debug, Args)]
pub struct StartSession {
    /// Title of the session (if empty, basename of directory is used)
    pub title: Option<String>,

    /// Directory to start the tmux session in
    #[arg(short = 'd', default_value = ".")]
    pub directory: PathBuf,

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

pub fn start_session (_config: Config, session: &StartSession) ->  Session
{
    let directory = fs::canonicalize(session.directory.clone())
        .unwrap_or_else(|_| {
            eprintln!("Invalid directory");
            std::process::exit(1);
    });

    env::set_current_dir(&directory).expect("Failed to change directory");

    let title = match session.title.clone() {
        Some(name) => name.clone(),
        None=> directory
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string() 
    };

    Session {
        windows: default_windows(),
        title,
        path: "".to_string(),
        git: false,
        attach: !session.no_attach,
    }
}
