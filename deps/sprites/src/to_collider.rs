use gm::flat::Shape;
use rapier2d::prelude::ColliderBuilder;

pub trait ToCollider {
    fn to_collider(&self) -> ColliderBuilder;
}

impl ToCollider for Shape {
    fn to_collider(&self) -> ColliderBuilder {
        match self {
            Shape::Rect(size) => ColliderBuilder::cuboid(size.width, size.height),
            Shape::Circle(r) => ColliderBuilder::ball(*r),
            Shape::Triangle(a, b, c) => {
                ColliderBuilder::triangle([a.x, a.y].into(), [b.x, b.y].into(), [c.x, c.y].into())
            }
        }
    }
}
