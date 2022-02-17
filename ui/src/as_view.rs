use std::{
    ops::DerefMut,
    slice::{Iter, IterMut},
};

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

impl AsView for &mut Rglica<dyn View> {
    fn as_view(&mut self) -> &mut dyn View {
        (**self).deref_mut()
    }
}

pub trait HasIterMut {
    type Item;
    fn iter_mut(&mut self) -> IterMut<'_, Self::Item>;
}

impl<T, const N: usize> HasIterMut for [T; N] {
    type Item = T;

    fn iter_mut(&mut self) -> IterMut<'_, Self::Item> {
        self.iter_mut()
    }
}

impl<T> HasIterMut for [T] {
    type Item = T;

    fn iter_mut(&mut self) -> IterMut<'_, Self::Item> {
        self.iter_mut()
    }
}

impl<T> HasIterMut for &mut [T] {
    type Item = T;

    fn iter_mut(&mut self) -> IterMut<'_, Self::Item> {
        self.iter_mut()
    }
}
