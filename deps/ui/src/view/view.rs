use std::fmt::Debug;

use rtools::Boxed;

use crate::ViewBase;

pub trait View: Boxed + Debug {
    fn setup(&mut self) {}

    fn layout(&mut self) {}

    fn update(&mut self) {}

    fn view(&self) -> &ViewBase;
    fn view_mut(&mut self) -> &mut ViewBase;
}
