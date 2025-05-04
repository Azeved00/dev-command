use clap::Parser;
use std::{
    env, fs,
    path::PathBuf,
    process::Command,
};

mod config;
use config::{
    Session,
    Config,
    default_windows,
};

const DEFAULT_CONFIG_PATH:&str = "$XDG_CONFIG_HOME/dev/config.toml";


#[derive(Parser)]
#[command(version, about = "Launch a dev environment with tmux and nix", long_about = None)]
struct Cli {
    /// Directory to start the tmux session in
    #[arg(short = 'd', default_value=".")]
    directory: PathBuf,

    /// Set a session title,
    /// If no title is given then the basename of the directory is used
    #[arg(short = 't', default_value="")]
    session_title: String,

    ///  Use a saved session from config
    #[arg(short = 'n')]
    session_name: Option<String>,

    /// Do not attach to the tmux server
    #[arg(short = 'a')]
    no_attach: bool,

    /// Initiate a nix shell
    #[arg(short = 'k')]
    nix_shell: bool,

    /// Rename the session based on the nix session
    #[arg(short = 'r')]
    nix_rename: bool,

    /// Give a custom config
    #[arg(short = 'c', long="config", default_value=DEFAULT_CONFIG_PATH)]
    config: PathBuf,
}

fn expand_env_vars(path: &str) -> PathBuf {
    let expanded = shellexpand::full(path).unwrap();
    PathBuf::from(expanded.as_ref())
}


fn initiate_tmux(session: Session){
    Command::new("tmux")
        .args(["new-session", "-d", "-s", &session.title])
        .spawn()
        .expect("Failed to start tmux session");




    for (idx, window) in session.windows.iter().enumerate() {
        if idx != 0 {
            Command::new("tmux")
                .args([
                    "new-window",
                    "-t", &format!("{}:{}", session.title, idx+1)])
                .status()
                .ok();
        }

        if window.title != "".to_string(){
            Command::new("tmux")
                .args([
                    "rename-window",
                    "-t", &format!("{}:{}", session.title, idx+1),
                    &window.title
                ])
                .status()
                .ok();
        }
        else if PathBuf::from("flake.nix").exists() {
            if window.nix_rename {
                let rename_cmd = r#"tmux rename-window "$(nix --quiet develop --quiet -c bash -c 'env | awk -F= '\''{ if ($1 == "name") print $2 }'\'')" ; clear"#;
                Command::new("tmux")
                    .args([
                        "send-keys", 
                        "-t", &format!("{}:{}", session.title, idx+1), rename_cmd, 
                        "Enter"])
                    .status()
                    .ok();
            }
        }

        if window.nix_shell != "".to_string() {
            Command::new("tmux")
                .args([
                    "send-keys", 
                    "-t", &format!("{}:{}", session.title, idx+1), 
                    &format!("nix develop --impure .#{}", window.nix_shell), 
                    "Enter"])
                .status()
                .ok();
        }
    }

    if session.attach {
        Command::new("tmux")
            .args(["switch-client", "-t", &session.title])
            .status()
            .ok();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli = Cli::parse();

    let b = cli.config.to_string_lossy();
    let expanded = shellexpand::full(&b).unwrap();
    let mut config = Config::get_config(PathBuf::from(expanded.as_ref()))?;


    let s = match cli.session_name {
        Some(name) => {
            let s = config.sessions.get_mut(&name).unwrap_or_else(|| {
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

            s.clone()
        }

        None => {
            let directory = fs::canonicalize(cli.directory.clone()).unwrap_or_else(|_| {
                eprintln!("Invalid directory");
                std::process::exit(1);
            });

            env::set_current_dir(&directory).expect("Failed to change directory");

            if cli.session_title == "" {
                cli.session_title = directory
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
            }

            Session {
                windows: default_windows(),
                title: cli.session_title,
                attach: !cli.no_attach,
                path: "".to_string(),
            }
        }
    };

    //println!("{:#?}", s);

    initiate_tmux(s);
    Ok(())
}
