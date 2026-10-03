pub mod commands;
pub mod config;

use std::{
    env,
    path::PathBuf,
    process::Command,
};
use clap::Parser;

use crate::config::{
    Session,
    Config,
    Window,
};
use crate::commands::{
    Cli,
    Commands,
    start::start_session,
    load::load_session,
    init::init_project,
};



pub fn expand_env_vars(path: &str) -> PathBuf {
    let expanded = shellexpand::full(path).unwrap();
    PathBuf::from(expanded.as_ref())
}

fn create_window(index: usize, window: &Window, session: &Session, cli: &Cli){
    if index != 1 {
        if cli.verbose >= 1 {
            println!("Creating window {}:{}", session.title, index);
        }
        Command::new("tmux")
            .args([
                "new-window",
                "-t", &format!("{}:{}", session.title, index)])
            .status()
            .ok();
    }

    if window.title != "".to_string(){
        if cli.verbose >= 1 {
            println!("Setting window title");
        }
        Command::new("tmux")
            .args([
                "rename-window",
                "-t", &format!("{}:{}", session.title, index),
                &window.title
            ])
            .status()
            .ok();
    }
    else if PathBuf::from("flake.nix").exists() {
        if window.nix_rename {
            if cli.verbose >= 1 {
                println!("Using nix to rename the window");
            }
            let rename_cmd = r#"tmux rename-window "$(nix --quiet develop --quiet -c bash -c 'env | awk -F= '\''{ if ($1 == "name") print $2 }'\'')" ; clear"#;
            Command::new("tmux")
                .args([
                    "send-keys", 
                    "-t", &format!("{}:{}", session.title, index), rename_cmd, 
                    "Enter"])
                .status()
                .ok();
        }
    }

    if window.nix_shell != "".to_string() {
        if cli.verbose >= 1 {
            println!("Starting nix shell");
        }

        if PathBuf::from("flake.nix").exists() {
            Command::new("tmux")
                .args([
                    "send-keys", 
                    "-t", &format!("{}:{}", session.title, index), 
                    &format!("nix develop --impure .#{}", window.nix_shell), 
                    "Enter"])
                .status()
                .ok();
        } 
        else if PathBuf::from("shell.nix").exists() {
            Command::new("tmux")
                .args([
                    "send-keys", 
                    "-t", &format!("{}:{}", session.title, index), 
                    &format!("nix-shell --impure"), 
                    "Enter"])
                .status()
                .ok();
        }
        else if PathBuf::from("default.nix").exists() {
            Command::new("tmux")
                .args([
                    "send-keys", 
                    "-t", &format!("{}:{}", session.title, index), 
                    &format!("nix-shell --impure"), 
                    "Enter"])
                .status()
                .ok();
        }
        else {
            eprintln!("No nix shell file detected");
        }
    }
}

fn attach_session(session: &Session, cli:&Cli){
    if session.attach {
        if env::var("TMUX").is_ok() {
            if cli.verbose >= 1 {
                println!("Already inside tmux, changig client");
            }
            Command::new("tmux")
                .args(["switch-client", "-t", &format!("{}:1", session.title)])
                .status()
                .ok();
        } else {
            if cli.verbose >= 1 {
                println!("Attaching to session");
            }
            Command::new("tmux")
                .args(["attach-session", "-t",&format!("{}:1", session.title)])
                .status()
                .ok();
        }
    }
}


fn initiate_tmux(session: Session, cli: Cli){
    if session.git{
        if cli.verbose >= 1 {
            println!("Fetching updates from git remote");
        }

        Command::new("git")
            .args(["fetch", "--all", "--prune"])
            .status()
            .ok();
    }

    if cli.verbose >= 1 {
        println!("Starting tmux session (detached)");
    }

    let x = Command::new("tmux")
        .args(["new-session", "-d", "-s", &session.title])
        .status().expect("failed to create new session");

    if !x.success(){
        println!("Session '{}' already exists or failed to create.", session.title);
        attach_session(&session, &cli);
        return ;
    }

    for (idx, window) in session.windows.iter().enumerate() {
        create_window(idx+1, &window, &session, &cli);
    }

    attach_session(&session, &cli);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    if cli.verbose >= 2 {
        println!("{:#?}", cli);
    }

    let b = cli.config.to_string_lossy();
    let expanded = shellexpand::full(&b).unwrap();
    let config = Config::get_config(PathBuf::from(expanded.as_ref()))?;

    if cli.verbose >= 1 {
        println!("Generating session object");
    }
    let s = match cli.command {
        Commands::Start(ref session) => start_session(config, session),
        Commands::Load(ref session) => load_session(config, session),
        Commands::Init(ref session) => init_project(config, session),
    };
    if cli.verbose >= 2 {
        println!("{:#?}", s);
    }

    if cli.verbose >= 1 {
        println!("Generating tmux session");
    }
    initiate_tmux(s, cli);
    Ok(())
}
