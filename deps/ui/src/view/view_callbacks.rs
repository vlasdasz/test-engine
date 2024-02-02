use gm::flat::Size;
use refs::{Own, Weak};

use crate::{view::view_frame::ViewFrame, View};

pub trait ViewCallbacks {
    fn update(&mut self);
    fn on_selection_changed(&mut self, selected: bool);
    fn content_size(&self) -> &Size;
}

impl<T: ?Sized + View> ViewCallbacks for T {
    default fn update(&mut self) {}
    default fn on_selection_changed(&mut self, _: bool) {}
    default fn content_size(&self) -> &Size {
        &self.frame().size
    }
}

pub trait ViewInternalSetup {
    fn __internal_setup(&mut self);
}

pub trait ViewSetup {
    fn new() -> Own<Self>
    where Self: Default;
    fn setup(self: Weak<Self>);
}

impl<T: View + 'static> ViewSetup for T {
    default fn new() -> Own<Self>
    where Self: Default {
        Own::<Self>::default()
    }
    default fn setup(self: Weak<Self>) {}
}
