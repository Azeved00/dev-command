use std::path::{Path, PathBuf};
use std::io;
use std::fs;
use std::env;
use clap::Args;
use include_dir::{include_dir, Dir};

use crate::config::{
    Session,
    Config,
};
use crate::commands::start::{
    StartSession, start_session,
};

const DEFAULT_TEMPLATE: &str = "default";
static TEMPLATES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");


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
        .unwrap_or_else(|err| {
            eprintln!("{}",err);
            eprintln!("Failed to create project directory");
            std::process::exit(1);
    });

    let target = if command.directory.is_absolute() {
        command.directory.clone()
    } else {
        std::env::current_dir().unwrap().join(&command.directory)
    };

    copy_template(&command.template, &target)
        .unwrap_or_else(|err| {
            eprintln!("{}",err);
            eprintln!("Failed to instantiate the template");
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


fn copy_template(template_name: &str, target: &Path) -> io::Result<()> {
    let template_dir = TEMPLATES_DIR
        .get_dir(template_name.trim())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("template not found: {template_name:?}"),
            )
        })?;

    for file in template_dir.files() {
        let relative_path = file
            .path()
            .strip_prefix(template_dir.path())
            .map_err(io::Error::other)?;

        let output_path = target.join(relative_path);

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&output_path, file.contents()).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("failed to write {}: {error}", output_path.display()),
            )
        })?;
    }

    for directory in template_dir.dirs() {
        let relative_path = directory
            .path()
            .strip_prefix(template_dir.path())
            .map_err(io::Error::other)?;

        fs::create_dir_all(target.join(relative_path))?;
    }

    Ok(())
}
