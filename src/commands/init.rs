use clap::Args;
use std::path::PathBuf;
use std::fs;
use std::env;

use crate::config::{
    Session,
    Config,
    default_windows,
};
use crate::commands::start::{
    StartSession, start_session,
};

/// Create a new tmux session from a directory
#[derive(Debug, Args)]
pub struct InitProject {
    /// Directory to create the project on 
    pub directory: PathBuf,

    /// Title of the session (if empty, basename of directory is used)
    #[arg(short = 't')]
    pub title: Option<String>,

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

pub fn init_project (config: Config, command: &InitProject) ->  Session
{
    fs::create_dir_all(command.directory.clone())
        .unwrap_or_else(|_| {
            eprintln!("Failed to create project directory");
            std::process::exit(1);
    });
    let mut readme = command.directory.clone();
    readme.push("readme.md");

    fs::write(readme, "Application started\n")
        .unwrap_or_else(|_| {
            eprintln!("Failed to create readme");
            std::process::exit(1);
    });

    let start_command : StartSession = StartSession {
        title: command.title.clone(),
        directory: command.directory.clone(),
        no_attach: command.no_attach.clone(),
        nix_shell: command.nix_shell.clone(),
        nix_rename: command.nix_rename.clone(),

    };
    start_session(config, &start_command)
}
