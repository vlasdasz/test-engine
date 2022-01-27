use std::process::Command;


fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_os.as_str() {
        "android" => {}
        "ios" => {}
        "linux" => {},
        _ => {}
    };
}
