use std::ops::{Deref, DerefMut};

extern crate gm;

struct Fok<T> {
    value: T,
}

impl<T> Fok<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> Deref for Fok<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> DerefMut for Fok<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

trait Soka {
    fn sok(&self);
}

#[derive(Default)]
struct Kal {
    pub val: u32,
}

impl Soka for Kal {
    fn sok(&self) {
        dbg!("sok");
    }
}

fn main() {
    let skidel: Fok<Kal> = Fok::new(Kal::default());

    skidel.sok();
    dbg!(skidel.val);

    //dbg!(skidel.sok());

    // let sok: Rect = (1, 2, 3, 4).into();
    //
    // dbg!(&sok);
    //
    // let mut own = Own::from(sok);
    //
    // dbg!(&own);
    //
    // own.origin.x += 20.0;
    //
    // dbg!(&own);
}
