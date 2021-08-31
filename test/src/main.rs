use std::ops::{Deref, DerefMut};
use std::cell::RefCell;
use std::borrow::Borrow;

extern crate gm;

struct Rglica<T: ?Sized> {
    pub ptr: *mut T
}

impl<T: ?Sized> Rglica<T> {
    pub fn from_box(bx: &mut Box<T>) -> Self {
        Self {
            ptr: &mut **bx
        }
    }
}

impl<T: ?Sized> Deref for Rglica<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            self.ptr.as_ref().unwrap()
        }
    }
}

impl<T: ?Sized> DerefMut for Rglica<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.ptr.as_mut().unwrap()
        }
    }
}

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

    unsafe {
        pookto.ptr.as_mut().unwrap().sok();
    }

    unsafe {
        skidrow.ptr.as_mut().unwrap().sok();
    }
}
