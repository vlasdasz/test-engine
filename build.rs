
extern crate dirs;

use dirs::home_dir;
use std::process::{Command, Output};
use std::path::PathBuf;

fn output_to_str(output: Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn repo_status(path: &PathBuf) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("status")
        .arg("-s")
        .output()
        .expect("git command failed");

    output_to_str(output)
}

fn is_git_repo(path: &PathBuf) -> bool {
    path.join(".git").exists()
}

fn has_changes(path: &PathBuf) -> bool {
    repo_status(path).len() > 0
}

fn pull(path: &PathBuf) {
    if !is_git_repo(path) {
        panic!("{:?} is not a git repo", path);
    }
    if has_changes(path) {
        //panic!("Repo {:?} has changes", path);
        return;
    }
    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("pull")
        .output()
        .expect("git command failed");
    println!("cargo:warning=pull {:?} {}", path, output_to_str(output))
}

fn clone(remote: &PathBuf, local: &PathBuf) {
    if local.exists() {
        panic!("Can't clone into {:?} already exists", local)
    }
    let output = Command::new("git")
        .arg("clone")
        .arg(remote)
        .arg(local)
        .output()
        .expect("git command failed");
    println!("cargo:warning=cloning {:?} {}", remote, output_to_str(output))
}

fn pwd() {
    let output = Command::new("pwd")
        .output()
        .expect("git command failed");
    println!("cargo:warning=pwd {}", output_to_str(output))
}

fn link_deps() {
    let output = Command::new("ln")
        .arg("-s")
        .arg("~/.rdeps/tools")
        .output()
        .expect("ln command failed");
    println!("cargo:warning=ln {}", output_to_str(output))
}

fn main() {
    println!("cargo:rustc-link-lib=framework=OpenGLES");
}