pub mod basic;
pub mod complex;
pub mod input;
pub mod layout;
pub mod text;
pub mod view;

pub use basic::ImageView;
pub use basic::Label;
pub use complex::DPadView;
pub use input::Touch;
pub use layout::Layout;
pub use text::{Font, Glyph};
pub use view::*;

use lazy_static::lazy_static;

use std::path::PathBuf;
use std::sync::Mutex;

lazy_static! {
    pub static ref DEFAULT_FONT_PATH: Mutex<PathBuf> = Mutex::new(PathBuf::new());
    pub static ref DEFAULT_FONT: Mutex<Font> =
        Mutex::new(Font::new(&DEFAULT_FONT_PATH.lock().unwrap(), 48).unwrap());
}
