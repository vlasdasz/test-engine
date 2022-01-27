use std::process::Command;

fn linux_setup() {
    Command::new("sudo")
        .arg("apt")
        .arg("update")
        .output()
        .unwrap();

    Command::new("sudo")
        .arg("apt")
        .arg("-y")
        .arg("install")
        .arg("cmake")
        .arg("mesa-common-dev")
        .arg("libgl1-mesa-dev")
        .arg("libglu1-mesa-dev")
        .arg("xorg-dev")
        .output()
        .unwrap();
}

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_os.as_str() {
        "android" => {}
        "ios" => {}
        "linux" => linux_setup(),
        _ => {}
    };
}
