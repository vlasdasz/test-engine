use gm::Rect;
use std::alloc::alloc;
use std::alloc::Layout;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::ptr::{null, null_mut};

extern crate gm;

struct Own<T: Debug> {
    pointer: *mut T,
}

impl<T: Debug> Own<T> {
    pub fn from(value: T) -> Self {
        Self {
            pointer: unsafe {
                let ptr = alloc(Layout::new::<T>()) as *mut T;
                *ptr = value;
                ptr
            },
        }
    }
}

impl<T: Debug> Deref for Own<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.pointer }
    }
}

impl<T: Debug> DerefMut for Own<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.pointer }
    }
}

impl<T: Debug> Debug for Own<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

struct Fer<T> {
    pointer: *mut T,
}

// impl<T> Fer<T> {
//     pub fn from()
// }

struct View {
    pub superview: *const View,
    pub subviews: Vec<Box<View>>,
}

impl View {
    pub fn new() -> Self {
        Self {
            superview: null(),
            subviews: vec![],
        }
    }

    //pub fn add_subview()
}

fn main() {
    let sok: Rect = (1, 2, 3, 4).into();

    dbg!(&sok);

    let mut own = Own::from(sok);

    dbg!(&own);

    own.origin.x += 20.0;

    dbg!(&own);
}
