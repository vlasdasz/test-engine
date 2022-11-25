use std::ops::Deref;
use std::path::Path;
use test_engine::Image;

use rtools::data_manager::{LoadFromPath, Managed};
use rtools::data_manager::DataStorage;
use test_engine::text::{render_text, Font};
use ui::{view, ViewCallbacks};
use rtools::data_manager::DataManager;
use rtools::managed;

#[view]
#[derive(Default)]
pub struct EmptyView {}

struct Test {
    _int: i32
}

managed!(Test);

impl LoadFromPath for Test {
    fn load(_path: &Path) -> Self {
        Self { _int: 10 }
    }
}

impl ViewCallbacks for EmptyView {
    fn update(&mut self) {
        let mut image = render_text("SPEspgfdjsklfjdslkfjklds", &Font::san_francisco().deref(), 16);
        image.free();
    }
}
