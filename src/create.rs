use clap::{Arg, Command};
use std::fs;
use std::path::Path;
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use std::process::Command as ShellCommand;

struct Project{
    path
        dirname
        lang
}

/// Returns `true` if we have internet
fn has_internet() -> bool {
    reqwest::blocking::get("http://example.com").is_ok()
}


fn create_project(path: &str, lang: Option<&String>, online: bool) {
    let path_obj = Path::new(path);
    if !path_obj.exists() {
        fs::create_dir_all(path_obj).expect("Failed to create directory");
        println!("Created directory at {}", path);
    }

    if has_internet() && online {
        if let Some(lang_name) = lang {
            download_and_copy_template(path_obj, lang_name);
        } else {
            println!("Online, but no language specified — skipping remote template.");
        }
    } else {
        println!("No internet — using local fallback template.");
        let fallback_path = "templates/default";
        let options = CopyOptions::new();
        copy_dir(fallback_path, path_obj, &options).expect("Failed to copy default template");
    }
}

/// Example stub for downloading a template
fn download_and_copy_template(target_dir: &Path, lang: &str) {
    // Example GitHub template repo
    let repo_url = format!("https://github.com/youruser/{}_template.git", lang);
    let temp_dir = format!("/tmp/{}_template", lang);

    println!("Cloning {} to {}", repo_url, temp_dir);

    // Use `git` command to clone
    let status = ShellCommand::new("git")
        .args(["clone", "--depth", "1", &repo_url, &temp_dir])
        .status()
        .expect("Failed to run git clone");

    if status.success() {
        let options = CopyOptions::new();
        copy_dir(&temp_dir, target_dir, &options).expect("Failed to copy cloned template");
    } else {
        println!("Failed to clone template. Using fallback.");
    }
}
