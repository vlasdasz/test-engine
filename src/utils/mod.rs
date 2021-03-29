
#[macro_use] pub mod log;

pub mod array_view;
pub mod platform;
pub mod regex;

pub use platform::Platform;

pub use array_view::ArrayView;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub type Shared<T> = Rc<RefCell<T>>;
pub type MutWeak<T> = Weak<RefCell<T>>;

pub fn make_shared<T>(val: T) -> Shared<T> {
    Rc::new(RefCell::new(val))
}