use refs::Weak;

use crate::View;

pub trait ViewCallbacks {
    fn update(&mut self);
    fn on_selection_changed(&mut self, selected: bool);
}

impl<T: ?Sized + View> ViewCallbacks for T {
    default fn update(&mut self) {}
    default fn on_selection_changed(&mut self, _: bool) {}
}

pub trait ViewInternalSetup {
    fn internal_setup(&mut self);
}

pub trait ViewSetup {
    fn setup(self: Weak<Self>);
}

impl<T: View> ViewSetup for T {
    default fn setup(self: Weak<Self>) {}
}
