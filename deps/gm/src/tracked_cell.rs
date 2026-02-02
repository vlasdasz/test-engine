use std::{
    cell::{Ref, RefCell, RefMut},
    panic::Location,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TrackedCell<T> {
    inner:       RefCell<T>,
    borrowed_at: RefCell<Option<&'static Location<'static>>>,
}

impl<T> TrackedCell<T> {
    pub fn new(val: T) -> Self {
        Self {
            inner:       RefCell::new(val),
            borrowed_at: RefCell::new(None),
        }
    }

    #[track_caller]
    pub fn borrow(&self) -> Ref<'_, T> {
        match self.inner.try_borrow() {
            Ok(b) => {
                *self.borrowed_at.borrow_mut() = Some(Location::caller());
                b
            }
            Err(_) => {
                let loc = self.borrowed_at.borrow().unwrap();
                panic!("Already borrowed at: {}:{}", loc.file(), loc.line());
            }
        }
    }

    #[track_caller]
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        match self.inner.try_borrow_mut() {
            Ok(b) => {
                *self.borrowed_at.borrow_mut() = Some(Location::caller());
                b
            }
            Err(_) => {
                let loc = self.borrowed_at.borrow().expect("Unknown location");
                panic!(
                    "Conflicting borrow! Already borrowed at: {}:{}",
                    loc.file(),
                    loc.line()
                );
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    #[should_panic(expected = "Conflicting borrow! Already borrowed at: deps/gm/src/tracked_cell.rs:63")]
    fn test_tracked_cell() {
        let cell = TrackedCell::new(5);

        let _a = cell.borrow_mut();
        let _b = cell.borrow_mut();
    }
}
