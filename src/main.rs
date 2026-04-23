use std::{
    env, fs,
    path::PathBuf,
    process::Command,
};
mod cli;
use cli::Cli;
use clap::Parser;

mod config;
use config::{
    Session,
    Config,
    default_windows,
    Window,
};



fn expand_env_vars(path: &str) -> PathBuf {
    let expanded = shellexpand::full(path).unwrap();
    PathBuf::from(expanded.as_ref())
}
fn tmux(args: &[&str]) {
    Command::new("tmux")
        .args(args)
        .status()
        .ok();
}

fn rename_window(index: usize, window: &Window, session: &Session, cli: &Cli){
    if !window.title.is_empty(){
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
}
fn setup_panes(index: usize, window: &Window, session: &Session, cli: &Cli) {
    let target = format!("{}:{}", session.title, index);

    for i in 1..window.pane_count {
        if cli.verbose >= 1 {
            println!("Creating pane {} in window {}", i, target);
        }

        tmux(&["split-window", "-t", &target]);
    }


    if cli.verbose >= 1 {
        println!("Applying layout: {}", window.pane_layout);
    }

    tmux(&[
        "select-layout",
        "-t",
        &target,
        &window.pane_layout,
    ]);
}

fn start_nix_shells(index: usize, window: &Window, session: &Session, cli: &Cli){
    if window.nix_shell.is_empty() {
        return;
    }

    if cli.verbose >= 1 {
        println!("Starting nix shell in all panes");
    }

    let target = format!("{}:{}", session.title, index);

    // Decide nix command once
    let nix_cmd = if PathBuf::from("flake.nix").exists() {
        format!("nix develop --impure .#{}", window.nix_shell)
    } else if PathBuf::from("shell.nix").exists() 
        || PathBuf::from("default.nix").exists() 
    {
        "nix-shell --impure".to_string()
    } else {
        eprintln!("No nix shell file detected");
        return;
    };

    // Get all pane IDs in this window
    let output = Command::new("tmux")
        .args(["list-panes", "-t", &target, "-F", "#{pane_id}"])
        .output()
        .expect("failed to list panes");

    let panes = String::from_utf8_lossy(&output.stdout);

    // Send nix command to each pane
    for pane in panes.lines() {
        if cli.verbose >= 1 {
            println!("Starting nix shell in pane {}", pane);
        }

        Command::new("tmux")
            .args([
                "send-keys",
                "-t",
                pane,
                &nix_cmd,
                "Enter",
            ])
            .status()
            .ok();
    }
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

    rename_window(index, window, session, cli);
    setup_panes(index, window, session, cli);
    start_nix_shells(index, window, session, cli);
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
    let mut config = Config::get_config(PathBuf::from(expanded.as_ref()))?;

    if cli.verbose >= 1 {
        println!("Generating session object");
    }
    let s = match cli.command {
        cli::Commands::Load(ref session) => {
            let s = config.sessions.get_mut(&session.name).unwrap_or_else(|| {
                eprintln!("No session with that name in the config file");
                std::process::exit(1);
            });

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

        cli::Commands::New(ref session) => {
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
