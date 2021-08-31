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
    let mut sepel_old: Box<dyn Soka> = Box::new(Kal::default());
    sepel_old.sok();

    let mut skogol_old: Box<Kal> = Box::new(Kal::default());
    skogol_old.sok();
    dbg!(skogol_old.val);

    let mut pookto = Rglica::from_box(&mut sepel_old);

    let mut skidrow = Rglica::from_box(&mut skogol_old);

    dbg!(skidrow.val);

    pookto.sok();
    skidrow.sok();
}
