use cfg_aliases::cfg_aliases;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(ios)");
    println!("cargo::rustc-check-cfg=cfg(android)");
    println!("cargo::rustc-check-cfg=cfg(mobile)");
    println!("cargo::rustc-check-cfg=cfg(desktop)");

    cfg_aliases! {
        android: { target_os = "android" },
        ios:     { target_os = "ios" },

        macos:   { target_os = "macos" },
        linux:   { target_os = "linux" },
        freebsd: { target_os = "freebsd" },

        mobile:  { any(    target_os = "android", target_os = "ios")  },
        desktop: { not(any(target_os = "android", target_os = "ios")) },

        not_android: { not(target_os = "android") },
    }
}
