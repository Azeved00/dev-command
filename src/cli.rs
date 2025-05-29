use std::path::PathBuf;
use clap::{
    self,
    Parser,
};

pub const DEFAULT_CONFIG_PATH:&str = "$XDG_CONFIG_HOME/dev/config.toml";


#[derive(Parser)]
#[command(version, about = "Launch a dev environment with tmux and nix", long_about = None)]
pub struct Cli {
    /// Directory to start the tmux session in
    #[arg(short = 'd', default_value=".")]
    pub directory: PathBuf,

    /// Set a session title,
    /// If no title is given then the basename of the directory is used
    #[arg(short = 't', default_value="")]
    pub session_title: String,

    ///  Use a saved session from config
    #[arg(short = 'n')]
    pub session_name: Option<String>,

    /// Do not attach to the tmux server
    #[arg(short = 'a')]
    pub no_attach: bool,

    /// Initiate a nix shell
    #[arg(short = 'k')]
    pub nix_shell: bool,

    /// Rename the session based on the nix session
    #[arg(short = 'r')]
    pub nix_rename: bool,

    /// Give a custom config
    #[arg(short = 'c', long="config", default_value=DEFAULT_CONFIG_PATH)]
    pub config: PathBuf,

    /// Increase output verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
