use educe::Educe;
use gm::{
    Color,
    flat::{Point, PointsPath, Shape, Size},
};
use refs::Weak;
use vents::Event;
use window::{VertexBuffer, image::Image};

use crate::Sprite;

#[derive(Educe)]
#[educe(Default)]
pub struct SpriteData {
    pub(crate) position: Point,

    pub(crate) size:        Size,
    pub(crate) render_size: Size,
    pub(crate) rotation:    f32,
    pub(crate) is_selected: bool,

    pub(crate) collision_enabled: bool,

    pub tag: u32,

    #[educe(Default = Color::random())]
    pub color: Color,

    #[educe(Default = 0.85)]
    pub z_position: f32,

    pub image:        Weak<Image>,
    pub on_collision: Event<Weak<dyn Sprite>>,

    pub vertex_buffer: Option<VertexBuffer>,
}

impl SpriteData {
    pub fn make(shape: Shape, position: Point) -> Self {
        Self {
            position,
            size: shape.size(),
            // Rapier rect size defined by half side
            render_size: if shape.is_rect() {
                shape.size() / 2.0
            } else {
                shape.size()
            },
            vertex_buffer: Self::shape_to_buffer(shape),
            ..Default::default()
        }
    }
}

impl SpriteData {
    fn shape_to_buffer(shape: Shape) -> Option<VertexBuffer> {
        match shape {
            Shape::Circle(_) | Shape::Rect(_) => None,
            Shape::Triangle(a, b, c) => Some(vec![a, b, c].into()),
            Shape::Polygon(points) | Shape::Polyline(points) => {
                let (vertices, indices) = PointsPath::tessellate(points);
                VertexBuffer {
                    vertices,
                    indices: indices.into(),
                }
                .into()
            }
        }
    }
}
