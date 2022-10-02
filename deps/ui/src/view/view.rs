use std::ops::{Deref, DerefMut};

use rtools::Weak;

use crate::{ViewBase, ViewCallbacks};

pub trait View: ViewCallbacks + Deref<Target = ViewBase> + DerefMut<Target = ViewBase> {
    fn init_views(&mut self);
    fn weak_view(&self) -> Weak<dyn View>;
}

#[derive(Default)]
pub struct SubView<T: View>(Weak<T>);

impl<T: View> Copy for SubView<T> {}

impl<T: View> Clone for SubView<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: View> Deref for SubView<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0.deref()
    }
}

impl<T: View> DerefMut for SubView<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

impl<T: View> From<Weak<T>> for SubView<T> {
    fn from(r: Weak<T>) -> Self {
        Self(r)
    }
}
