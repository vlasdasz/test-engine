use gm::flat::Size;
use refs::{Own, Weak};
use window::RenderPass;

use crate::{View, view::view_frame::ViewFrame};

pub trait ViewCallbacks {
    fn update(&mut self);
    fn before_render(&self, pass: &mut RenderPass);
    fn content_size(&self) -> &Size;
}

impl<T: ?Sized + View> ViewCallbacks for T {
    default fn update(&mut self) {}
    default fn before_render(&self, _pass: &mut RenderPass) {}
    default fn content_size(&self) -> &Size {
        &self.frame().size
    }
}

pub trait __ViewInternalSetup {
    fn __internal_before_setup(&mut self);
    fn __internal_setup(&mut self);
    fn __internal_inspect(&mut self);
    fn __internal_on_selection_changed(&mut self, selected: bool);
}

pub trait Setup {
    fn new() -> Own<Self>
    where Self: Default;
    fn setup(self: Weak<Self>);
    fn on_selection_changed(self: Weak<Self>, selected: bool);
    fn before_setup(self: Weak<Self>);
    fn inspect(self: Weak<Self>);
}

impl<T: View + 'static> Setup for T {
    default fn new() -> Own<Self>
    where Self: Default {
        Own::<Self>::default()
    }

    default fn setup(self: Weak<Self>) {}

    default fn on_selection_changed(self: Weak<Self>, _selected: bool) {}

    default fn before_setup(self: Weak<Self>) {}
    default fn inspect(self: Weak<Self>) {}
}
