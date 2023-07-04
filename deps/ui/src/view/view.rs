use std::{
    any::Any,
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
};

use refs::Weak;

use crate::{view::view_callbacks::ViewInternalSetup, ViewBase, ViewCallbacks};

pub trait View:
    ViewCallbacks + ViewInternalSetup + Deref<Target = ViewBase> + DerefMut<Target = ViewBase> + Debug {
    fn init_views(&mut self);
    fn base(&self) -> &ViewBase;
    fn weak_view(&self) -> Weak<dyn View>;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Default)]
pub struct SubView<T: View>(Weak<T>);

impl<T: View> SubView<T> {
    pub fn weak(&self) -> Weak<T> {
        self.0
    }
}

impl<T: View> Copy for SubView<T> {}

impl<T: View> Clone for SubView<T> {
    fn clone(&self) -> Self {
        *self
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

impl<T: View> Debug for SubView<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&"Subview", f)
    }
}

impl<T: View + ToString> ToString for SubView<T> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<T: View + AsRef<bool>> AsRef<bool> for SubView<T> {
    fn as_ref(&self) -> &bool {
        self.0.as_ref()
    }
}
