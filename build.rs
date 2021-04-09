
fn main() {
    println!("cargo:rustc-link-lib=framework=OpenGLES");
    println!("cargo:rustc-link-lib=framework=Kook");
}