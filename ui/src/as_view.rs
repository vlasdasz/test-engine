use std::ops::DerefMut;
use rtools::Rglica;
use crate::View;

pub trait AsView {
    fn as_view(&mut self) -> &mut dyn View;
}

impl AsView for Box<dyn View> {
    fn as_view(&mut self) -> &mut dyn View {
        self.deref_mut()
    }
}

impl AsView for &mut Box<dyn View> {
    fn as_view(&mut self) -> &mut dyn View {
        (**self).deref_mut()
    }
}

impl AsView for Rglica<dyn View> {
    fn as_view(&mut self) -> &mut dyn View {
        self.deref_mut()
    }
}