
#[macro_use] pub mod log;

pub mod array_view;
pub mod regex;

pub use array_view::ArrayView;
use std::rc::Rc;
use std::cell::RefCell;

pub type Shared<T> = Rc<RefCell<T>>;

pub fn make_shared<T>(val: T) -> Shared<T> {
    Rc::new(RefCell::new(val))
}