mod app;
pub mod image;
mod text;
pub mod utils;
mod wgpu_app;

mod frame_counter;
mod render;

pub use app::*;
use gm::{flat::Point, volume::UIVertex};
pub use render::wgpu_drawer::WGPUDrawer;
pub use text::*;
pub use wgpu;
pub use wgpu_app::*;
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
