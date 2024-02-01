pub mod app;
pub mod colored_image_state;
pub mod image;
mod pipeline;
pub mod rect_state;
pub mod state;
pub mod text;
pub mod utils;
pub mod vertex_layout;
pub mod wgpu_app;
pub mod wgpu_drawer;

use gm::{flat::Point, volume::UIVertex};
pub use wgpu;

pub(crate) const IMAGE_VERTICES: &[UIVertex] = &[
    UIVertex {
        pos: Point::new(-1.0, 1.0),
        uv:  Point::new(0.0, 0.0),
    },
    UIVertex {
        pos: Point::new(-1.0, -1.0),
        uv:  Point::new(0.0, 1.0),
    },
    UIVertex {
        pos: Point::new(1.0, 1.0),
        uv:  Point::new(1.0, 0.0),
    },
    UIVertex {
        pos: Point::new(1.0, -1.0),
        uv:  Point::new(1.0, 1.0),
    },
];
