use std::fmt::Debug;

use gm::Rect;
use serde::{Deserialize, Serialize};

#[typetag::serde(tag = "sokol")]
trait Sokol: Debug {
    fn frame(&self) -> &Rect;
}

#[derive(Debug, Serialize, Deserialize)]
struct Borba {
    frame: Rect,
}

#[typetag::serde(name = "borba")]
impl Sokol for Borba {
    fn frame(&self) -> &Rect {
        &self.frame
    }
}

fn main() {
    let data: Vec<Box<dyn Sokol>> = vec![Box::new(Borba {
        frame: (1, 2, 3, 4).into(),
    })];

    let stre = serde_json::to_string(&data).unwrap();

    println!("{}", stre);

    let data: Vec<Box<dyn Sokol>> = serde_json::from_str(&stre).unwrap();

    println!("{}", serde_json::to_string(&data).unwrap());
}
