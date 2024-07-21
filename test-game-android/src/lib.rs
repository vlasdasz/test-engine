#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: test_game::AndroidApp) {
    test_game::start_test_game(app);
}

use jni::{objects::JString, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_com_example_test_1game_MainActivity_helloFromJNI<'local>(
    mut env: JNIEnv<'local>,
    input: JString<'local>,
) {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let input: String = env.get_string(&input).expect("Couldn't get java string!").into();

    dbg!(&input);

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
}
