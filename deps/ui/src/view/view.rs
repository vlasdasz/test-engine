use std::ops::{Deref, DerefMut};

use rtools::{Boxed, Rglica};

use crate::{ViewBase, ViewCallbacks};

pub trait View: Boxed + ViewCallbacks + Deref<Target = ViewBase> + DerefMut<Target = ViewBase> {
    fn rglica(&self) -> Rglica<dyn View>;
}
