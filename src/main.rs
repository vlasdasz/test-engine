#![allow(dead_code)]

#[macro_use]
extern crate tools;
#[macro_use]
extern crate guard;

mod gm;
mod image;
mod te;
mod ui;
#[macro_use]
mod gl_wrapper;
mod sprites;

use crate::gl_wrapper::GLDrawer;
use crate::gm::Size;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tools::HasNew;

#[derive(Serialize, Deserialize)]
struct TestGest {
    pub i: u32,
    pub stro: String,
}

impl HasNew for TestGest {
    fn new() -> TestGest {
        TestGest {
            i: 100100,
            stro: "rglo".into(),
        }
    }
}

fn executable_name() -> String {
    std::env::current_exe()
        .ok()
        .expect("Failed to get std::env::current_exe()")
        .file_name()
        .expect("Failed to get executable name")
        .to_string_lossy()
        .into()
}

fn storage_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Failed to get home directory")
        .join(".".to_owned() + &executable_name())
}

fn set_value<T: Serialize>(value: &T, key: &str) {
    let json = serde_json::to_string(value).expect("Failed to serialize data");
    let dir = storage_dir();
    if !dir.exists() {
        fs::create_dir(&dir).expect("Failed to create dir")
    }
    fs::write(dir.join(key), json).expect("Failed to write to file");
}

fn get_value<'a, T: Serialize + Deserialize<'a> + HasNew>(key: &str) -> T {
    let dir = storage_dir();
    let path = dir.join(key);

    if !dir.exists() {
        fs::create_dir(dir).expect("Failed to create dir");
    }

    if !path.exists() {
        let new = T::new();
        set_value(&new, key);
        return new;
    }

    let file = fs::File::open(path)
        .expect("file should open read only");

    T::new()
}

fn main() {
    dbg!(executable_name());
    dbg!(storage_dir());

    let val = TestGest::new();

    set_value(&val, "sokol");

    return;
    GLDrawer::with_size(Size::make(1200, 600)).start_main_loop();
}
