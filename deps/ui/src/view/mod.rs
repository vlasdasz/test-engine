use std::fmt::Debug;

use rtools::Boxed;

use crate::{complex::Alert, view::view_internal::ViewInternal};

mod view_base;
mod view_data;
mod view_frame;
mod view_internal;
mod view_subviews;
mod view_touch;
mod view_touch_internal;

pub use view_base::ViewBase;
pub use view_data::ViewData;
pub use view_frame::ViewFrame;
pub use view_subviews::ViewSubviews;
pub use view_touch::ViewTouch;

pub trait View: Boxed + Debug {
    fn setup(&mut self) {}

    fn layout(&mut self) {}

    fn update(&mut self) {}

    fn view(&self) -> &ViewBase;
    fn view_mut(&mut self) -> &mut ViewBase;
}
