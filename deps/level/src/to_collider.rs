use std::panic::catch_unwind;

use gm::{checked_usize_to_u32, flat::Shape};
use rapier2d::{math::Real, parry::transformation::vhacd::VHACDParameters, prelude::ColliderBuilder};
use utils::{elapsed, Elapsed};

pub trait ToCollider {
    fn make_collider(&self) -> ColliderBuilder;
}

impl ToCollider for Shape {
    fn make_collider(&self) -> ColliderBuilder {
        match self {
            Shape::Rect(size) => ColliderBuilder::cuboid(size.width / 2.0, size.height / 2.0),
            Shape::Circle(r) => ColliderBuilder::ball(*r),
            Shape::Triangle(a, b, c) => {
                ColliderBuilder::triangle([a.x, a.y].into(), [b.x, b.y].into(), [c.x, c.y].into())
            }
            Shape::Polygon(points) => convex_collider(points),
            Shape::Polyline(points) => polyline_collider(points),
        }
    }
}

fn make_indices(points: &[gm::flat::Point]) -> (Vec<rapier2d::prelude::Point<Real>>, Vec<[u32; 2]>) {
    let points: Vec<_> = points.iter().map(|p| rapier2d::prelude::Point::<Real>::new(p.x, p.y)).collect();
    let indices: Vec<_> = (0..u32::try_from(points.len()).unwrap() - 1)
        .map(|i| [i, i + 1])
        .chain([[checked_usize_to_u32(points.len()) - 1, 0]])
        .collect();
    (points, indices)
}

fn polyline_collider(points: &[gm::flat::Point]) -> ColliderBuilder {
    let (points, indices) = make_indices(points);
    ColliderBuilder::polyline(points, indices.into())
}

fn convex_collider(points: &[gm::flat::Point]) -> ColliderBuilder {
    let (points, _indices) = make_indices(points);
    ColliderBuilder::convex_hull(&points).expect("Can't convex hull")
}

// This thing is super heavy. Around 10 seconds for simple polygon in debug
// mode.
fn _concave_collider(points: &[gm::flat::Point]) -> ColliderBuilder {
    elapsed!("concave_collider");

    let (points, indices) = make_indices(points);

    let result = catch_unwind(|| {
        ColliderBuilder::convex_decomposition_with_params(
            &points,
            &indices,
            &VHACDParameters {
                concavity: 0.0,
                ..Default::default()
            },
        )
    });

    if result.is_err() {
        dbg!(points.iter().map(|p| gm::flat::Point::new(p.x, p.y)).collect::<Vec<_>>());
        panic!("a");
    }

    result.expect("Failed to do this convex stuff")
}
