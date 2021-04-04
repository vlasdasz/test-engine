
extern crate home;
use home::home_dir;
use std::process::Command;


fn main() {
    let home = home_dir().unwrap();
    let deps = home.join(".rdeps");

    let tools = deps.join("tools");

    println!("cargo:warning=KOK: {:?}", home);
    println!("cargo:warning=KOK: {:?}", deps);
    println!("cargo:warning=KOK: {:?}", tools);

    let output = Command::new("ls")
        .output().expect("ls command failed to start");

    println!("cargo:warning=KOK: {:?}", String::from_utf8_lossy(&output.stdout));

    eprintln!("HELLOOO");
}