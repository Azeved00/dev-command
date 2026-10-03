use clap::Args;
use std::path::{Path, PathBuf};
use std::io;
use std::fs;
use std::env;

use crate::config::{
    Session,
    Config,
};
use crate::commands::start::{
    StartSession, start_session,
};

const DEFAULT_TEMPLATE: &str = "default";

/// Create a new tmux session from a directory
#[derive(Debug, Args)]
pub struct InitProject {
    /// Directory to create the project on 
    pub directory: PathBuf,

    /// The template to use
    /// if no template is provided then the default is used
    #[arg(short = 't', default_value = DEFAULT_TEMPLATE)]
    pub template: String,

    /// Title of the session (if empty, basename of directory is used)
    #[arg()]
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

    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("templates/")
        .join(command.template.clone());

    copy_dir_contents(&source, &command.directory)
        .unwrap_or_else(|_| {
            eprintln!("Failed to create project directory");
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

fn copy_dir_contents(src: &Path, target: &Path) -> io::Result<()> {
    fs::create_dir_all(target)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());

        if source_path.is_dir() {
            copy_dir_contents(&source_path, &target_path)?;
        } else {
            fs::copy(&source_path, &target_path)?;
        }
    }

    Ok(())
}
