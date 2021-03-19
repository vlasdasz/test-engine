// Rust

mod gm;

use gm::Point;

fn main() {
    let point = Point { x: 5.0, y: 5.0 };
    let new_point = Point::new();
    println!(
        "The area of the rectangle is {} square pixels. A {}",
        point.kok(),
        new_point.kok()
    );
}