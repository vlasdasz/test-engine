use std::{ops::Deref, path::Path};

use rtools::{
    data_manager::{DataManager, DataStorage, LoadFromPath, Managed},
    managed,
};
use test_engine::text::{render_text, Font};
use ui::{view, ViewCallbacks};

#[view]
#[derive(Default)]
pub struct EmptyView {}

#[derive(Default)]
struct Test {
    _int:      i32,
    _strengol: Vec<String>,
}

managed!(Test);

impl LoadFromPath for Test {
    fn load(_path: &Path) -> Self {
        Self {
            _int:      10,
            _strengol: Default::default(),
        }
    }
}

impl ViewCallbacks for EmptyView {
    fn update(&mut self) {
        // for _ in 0..10000 {
        // let mut sok = Test::default();
        // for _ in 0..100 {
        //     sok._strengol.push(String::from("spes"));
        // }
        // let mut obj = Test::add_with_hash(u64::random(), Test::default());
        // obj.free();
        //
        let mut image = render_text("SPEspgfdjsklfjdslkfjklds", &Font::san_francisco().deref(), 16);
        image.free();

        // let mut image = Image::get("up.png");
        // image.free();
        // }
    }
}
