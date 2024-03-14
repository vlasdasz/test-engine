use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        android: { target_os = "android" },
        ios:     { target_os = "ios" },

        macos:   { target_os = "macos"   },
        linux:   { target_os = "linux"   },

        mobile:  { any(    target_os = "android", target_os = "ios")  },
        desktop: { not(any(target_os = "android", target_os = "ios")) },

        not_android: { not(target_os = "android") },
    }
}
