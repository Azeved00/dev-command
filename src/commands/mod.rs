use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub mod start;
pub mod load;
pub mod init;

use start::StartSession;
use load::LoadSession;
use init::InitProject;

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
    #[command(alias = "s")] 
    Start(StartSession),

    #[command(alias = "l", alias = "ld")] 
    Load(LoadSession),

    #[command(alias = "i")] 
    Init(InitProject),
}
