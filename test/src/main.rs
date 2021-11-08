use std::fmt::Debug;

use erased_serde::serialize_trait_object;
use gm::Rect;
use serde::{Deserialize, Serialize};

trait Sokol: erased_serde::Serialize + Debug {
    fn frame(&self) -> &Rect;
}

#[derive(Debug, Serialize, Deserialize)]
struct Borba {
    frame: Rect,
}

serialize_trait_object!(Sokol);

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

    // let data: Vec<Box<dyn Sokol>> = serde_json::from_str(&stre).unwrap();
    //
    // println!("{}", serde_json::to_string(&data).unwrap());
}
