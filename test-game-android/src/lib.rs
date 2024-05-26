#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: test_game::AndroidApp) {
    test_game::start_test_game(app);
}
