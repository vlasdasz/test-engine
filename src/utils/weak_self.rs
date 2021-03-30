use crate::utils::{MutWeak, Shared, make_shared};
use std::rc::{Rc, Weak};


pub trait HasWeakSelf<T> {

    type Shared = Shared<T>;
    type Weak = MutWeak<T>;

    fn new() -> T;
    fn new_shared() -> Self::Shared;
    fn weak(&self) -> Self::Weak;
}
