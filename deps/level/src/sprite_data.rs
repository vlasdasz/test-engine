use educe::Educe;
use gm::{
    flat::{Point, PointsPath, Shape, Size},
    Color,
};
use refs::Weak;
use vents::Event;
use wgpu_wrapper::{image::Image, VertexBuffer};

use crate::Sprite;

#[derive(Educe)]
#[educe(Default)]
pub struct SpriteData {
    pub(crate) position: Point,

    pub(crate) size:        Size,
    pub(crate) rotation:    f32,
    pub(crate) is_selected: bool,

    pub(crate) collision_enabled: bool,

    pub tag: u32,

    #[educe(Default = Color::random())]
    pub color: Color,

    pub image:        Weak<Image>,
    pub on_collision: Event<Weak<dyn Sprite>>,

    pub vertex_buffer: Option<VertexBuffer>,
}

impl SpriteData {
    pub fn make(shape: Shape, position: Point) -> Self {
        Self {
            position,
            size: shape.size(),
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
            Shape::Convex(points) => Some(points.into()),
            Shape::Concave(points) => {
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
