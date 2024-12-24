mod input;
mod ui;
pub mod ui_test;
mod views;

pub use ::ui::*;
pub use gm::{
    Color, U8Color,
    flat::{Point, PointsPath, Rect, Size},
};
pub use input::*;
pub use ui::UI;
pub use ui_proc::view;
pub use views::color_meter::ColorMeter;
pub use window::{PolygonMode, Screenshot, image::Image, include_images};

pub use crate::ui::views::sprite_view::SpriteView;
