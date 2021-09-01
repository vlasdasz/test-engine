use tools::Rglica;

extern crate gm;

trait Soka {
    fn sok(&mut self);
}

#[derive(Default)]
struct Kal {
    pub val: u32,
}

impl Soka for Kal {
    fn sok(&mut self) {
        dbg!("sok");
    }
}

fn main() {

}
