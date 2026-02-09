mod input;
mod ui;
pub mod ui_test;
pub(crate) mod views;

pub use ::ui::*;
pub use gm::{
    color::*,
    flat::{Point, PointsPath, Rect, Size},
};
pub use input::*;
pub use ui::UIDrawer;
pub use ui_proc::view;
pub use views::{color_meter::ColorMeter, *};
pub use window::{
    PolygonMode, Screenshot,
    image::{Image, NoImage, Tinted},
};

pub use crate::ui::views::sprite_view::SpriteView;
