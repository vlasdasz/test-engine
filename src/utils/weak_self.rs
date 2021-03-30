use crate::utils::{MutWeak, Shared};

pub trait HasWeakSelf {
    fn new() -> Self where Self: Sized;
    fn new_shared() -> Shared<Self> where Self: Sized;
    fn weak(&self) -> MutWeak<Self> where Self: Sized;
}
