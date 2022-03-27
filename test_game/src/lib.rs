use std::{
    os::raw::{c_float, c_int, c_ulong},
    ptr,
};

use test_engine::app::App;

use crate::test_game_view::TestGameView;

mod test_game_level;
mod test_game_view;

static mut APP: *mut App<TestGameView> = ptr::null_mut();

#[no_mangle]
pub extern "C" fn create_screen(width: c_int, height: c_int) {
    unsafe { APP.as_mut().unwrap_unchecked().create_screen(width, height) }
}

#[no_mangle]
pub extern "C" fn set_screen_size(width: c_int, height: c_int) {
    unsafe { APP.as_mut().unwrap_unchecked().set_screen_size(width, height) }
}

#[no_mangle]
pub extern "C" fn update_screen() {
    unsafe { APP.as_mut().unwrap_unchecked().update_screen() }
}

#[no_mangle]
pub extern "C" fn on_touch(id: c_ulong, x: c_float, y: c_float, event: c_int) {
    unsafe { APP.as_mut().unwrap_unchecked().on_touch(id, x, y, event) }
}

#[no_mangle]
pub extern "C" fn set_monitor(
    ppi: c_int,
    scale: c_float,
    refresh_rate: c_int,
    resolution_x: c_int,
    resolution_y: c_int,
    width: c_float,
    height: c_float,
    diagonal: c_float,
) {
    unsafe {
        APP = Box::into_raw(Box::new(App::<TestGameView>::default()));
        APP.as_mut().unwrap_unchecked().set_monitor(
            ppi,
            scale,
            refresh_rate,
            resolution_x,
            resolution_y,
            width,
            height,
            diagonal,
        );
    }
}

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate android_logger;

    use android_logger::{Config, FilterBuilder};
    use gl_wrapper::monitor::Monitor;
    use log::Level;

    fn setup_logger() {
        android_logger::init_once(
            Config::default()
                .with_min_level(Level::Trace)
                .with_tag("test_engine")
                .with_filter(FilterBuilder::new().parse("debug,hello::crate=error").build()),
        );

        trace!("this is a verbose {}", "message");
        error!("setup_logger");
    }

    use android_ndk_sys::{jclass, jobject, JNIEnv};

    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MyGLRenderer_setup(_: JNIEnv, _: jclass) {
        create_screen(0, 0);
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MainActivity_setAssetManager(
        env: JNIEnv,
        _: jclass,
        asset_manager: jobject,
    ) {
        setup_logger();
        rtools::file::set_asset_manager(env, asset_manager);
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MyGLRenderer_setScreenSize(
        _: JNIEnv,
        _: jclass,
        width: c_int,
        height: c_int,
    ) {
        set_screen_size(width as _, height as _);
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MyGLRenderer_update(_: JNIEnv, _: jclass) {
        update_screen();
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MainActivity_onTouch(
        _: JNIEnv,
        _: jclass,
        id: c_ulong,
        x: c_float,
        y: c_float,
        event: c_int,
    ) {
        on_touch(id, x, y, event)
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MainActivity_setMonitor(
        _: JNIEnv,
        _: jclass,
        ppi: c_int,
        scale: c_float,
        refresh_rate: c_int,
        resolutionX: c_int,
        resolutionY: c_int,
        width: c_float,
        height: c_float,
        diagonal: c_float,
    ) {
        set_monitor(
            ppi,
            scale,
            refresh_rate,
            resolutionX,
            resolutionY,
            width,
            height,
            diagonal,
        )
    }
}
