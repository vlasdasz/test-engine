#![allow(incomplete_features)]
#![feature(default_free_fn)]

use std::{
    default::default,
    os::raw::{c_float, c_int, c_ulong},
    ptr,
};

use test_engine::{
    gm::Size,
    ui::{input::touch::Event, Touch},
    Screen,
};
use tools::Boxed;

use crate::test_view::TestView;

mod test_level;
mod test_view;

#[cfg(target_os = "android")]
#[macro_use]
extern crate log;

static mut SCREEN: *mut Screen = ptr::null_mut();

#[no_mangle]
pub extern "C" fn create_screen() {
    unsafe {
        SCREEN = Box::into_raw(Box::new(
            Screen::new(default())
                .add_view(TestView::boxed())
                .add_debug_view(),
        ));
    }
}

#[no_mangle]
pub extern "C" fn set_screen_size(width: c_float, height: c_float) {
    unsafe {
        SCREEN
            .as_mut()
            .unwrap_unchecked()
            .set_size(Size { width, height });
    }
}

#[no_mangle]
pub extern "C" fn update_screen() {
    unsafe {
        SCREEN.as_mut().unwrap_unchecked().update();
    }
}

#[no_mangle]
pub extern "C" fn on_touch(id: c_ulong, x: c_float, y: c_float, event: c_int) {
    #[allow(clippy::useless_conversion)]
    unsafe {
        SCREEN.as_mut().unwrap_unchecked().on_touch(Touch {
            id:       id.into(),
            position: (x * 2.0, y * 2.0).into(),
            event:    Event::from_int(event),
        })
    }
}

/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate android_logger;

    use android_logger::{Config, FilterBuilder};
    use log::Level;

    fn native_activity_create() {
        android_logger::init_once(
            Config::default()
                .with_min_level(Level::Trace) // limit log level
                .with_tag("test_engine") // logs will show under mytag tag
                .with_filter(
                    // configure messages for specific crate
                    FilterBuilder::new()
                        .parse("debug,hello::crate=error")
                        .build(),
                ),
        );

        trace!("this is a verbose {}", "message");
        error!("this is printed by default");
    }

    use android_ndk_sys::{jclass, jobject, JNIEnv};

    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MyGLRenderer_setup(_: JNIEnv, _: jclass) {
        create_screen();
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MainActivity_setAssetManager(
        env: JNIEnv,
        _: jclass,
        asset_manager: jobject,
    ) {
        native_activity_create();
        error!("figma?");
        error!("skibel {:?}", asset_manager);
        tools::file::set_asset_manager(env, asset_manager);
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
}
