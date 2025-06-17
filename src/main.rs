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
