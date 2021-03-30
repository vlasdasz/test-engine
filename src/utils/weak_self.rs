use crate::utils::{MutWeak, Shared, make_shared};
use std::rc::{Rc, Weak};


pub trait HasWeakSelf<T> {

    type Shared = Shared<T>;
    type Weak = MutWeak<T>;

    fn new() -> T;
    fn new_shared() -> Self::Shared;
    fn weak(&self) -> Self::Weak;
}

struct Kok {
    a: i8,
    weak: MutWeak<Kok>
}

impl HasWeakSelf<Kok> for Kok {

    fn new() -> Kok {
        Kok { a: 10, weak: Weak::new() }
    }

    fn new_shared() -> Self::Shared {
        let result = make_shared(Kok::new());
        result.try_borrow_mut().unwrap().weak = Rc::downgrade(&result);
        result
    }

    fn weak(&self) -> Self::Weak {
        self.weak.clone()
    }
}