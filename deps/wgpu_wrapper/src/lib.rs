mod app;
pub mod colored_image_state;
pub mod image;
mod pipeline;
pub mod rect_state;
pub mod state;
mod text;
pub mod utils;
pub mod vertex_layout;
mod wgpu_app;

mod wgpu_drawer;

pub use app::*;
use gm::{flat::Point, volume::UIVertex};
pub use text::*;
pub use wgpu;
pub use wgpu_app::*;
pub use wgpu_drawer::*;
pub use winit::event::{ElementState, MouseButton};

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
