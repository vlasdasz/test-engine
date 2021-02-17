// Rust

mod Point;

fn main() {
    let point = Point::Point { x: 5.0, y: 5.0 };
    let new_point = Point::Point::new();
    println!(
        "The area of the rectangle is {} square pixels.",
        point.kok()
    );
}