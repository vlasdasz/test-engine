fn main() {
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    println!("cargo:rustc-link-lib=framework=OpenGLES");
}
