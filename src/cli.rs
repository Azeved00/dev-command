use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;

pub const DEFAULT_CONFIG_PATH:&str = "$XDG_CONFIG_HOME/dev/config.toml";

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Launch a dev environment with tmux and nix",
    long_about = None
)]
pub struct Cli {
    /// Give a custom config
    #[arg(short = 'c', long = "config", default_value = DEFAULT_CONFIG_PATH)]
    pub config: PathBuf,

    /// Increase output verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(alias = "n")] 
    New(NewSession),

    #[command(alias = "l", alias = "ld")] 
    Load(LoadSession),
}

/// Create a new tmux session from a directory
#[derive(Debug, Args)]
pub struct NewSession {
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


