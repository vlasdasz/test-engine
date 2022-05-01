#![allow(clippy::module_inception)]

use crate::{complex::Alert, view::view_internal::ViewInternal};

mod view;
mod view_base;
mod view_callbacks;
mod view_data;
mod view_frame;
mod view_internal;
mod view_subviews;
mod view_touch;
mod view_touch_internal;

pub use view::View;
pub use view_base::ViewBase;
pub use view_callbacks::ViewCallbacks;
pub use view_data::ViewData;
pub use view_frame::ViewFrame;
pub use view_subviews::ViewSubviews;
pub use view_touch::ViewTouch;
