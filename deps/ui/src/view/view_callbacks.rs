use gm::flat::Size;
use refs::{Own, Weak};
use window::RenderPass;

use crate::{View, view::view_frame::ViewFrame};

pub trait ViewCallbacks {
    fn update(&mut self);
    fn before_render(&self, pass: &mut RenderPass);
    fn on_selection_changed(&mut self, selected: bool);
    fn content_size(&self) -> &Size;
}

impl<T: ?Sized + View> ViewCallbacks for T {
    default fn update(&mut self) {}
    default fn before_render(&self, _pass: &mut RenderPass) {}
    default fn on_selection_changed(&mut self, _: bool) {}
    default fn content_size(&self) -> &Size {
        &self.frame().size
    }
}

pub trait ViewInternalSetup {
    fn __internal_setup(&mut self);
}

pub trait Setup {
    fn new() -> Own<Self>
    where Self: Default;
    fn setup(self: Weak<Self>);
}

impl<T: View + 'static> Setup for T {
    default fn new() -> Own<Self>
    where Self: Default {
        Own::<Self>::default()
    }
    default fn setup(self: Weak<Self>) {}
}
