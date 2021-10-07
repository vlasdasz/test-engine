#![feature(default_free_fn)]

use std::{path::PathBuf, sync::Mutex};

pub use basic::{ImageView, Label};
pub use complex::DPadView;
pub use input::Touch;
use lazy_static::lazy_static;
pub use test::SubviewsTestView;
pub use text::{Font, Glyph};
pub use view::*;

pub mod basic;
pub mod complex;
pub mod input;
pub mod placer;
pub mod test;
pub mod text;
pub mod view;

lazy_static! {
    pub static ref DEFAULT_FONT_PATH: Mutex<PathBuf> = Mutex::new(PathBuf::new());
    pub static ref DEFAULT_FONT: Mutex<Font> =
        Mutex::new(Font::new(&DEFAULT_FONT_PATH.lock().unwrap(), 48).unwrap());
}
