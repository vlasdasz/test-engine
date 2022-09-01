use crate::View;

pub trait ViewCallbacks {
    fn setup(&mut self);
    fn update(&mut self);
}

impl<T: ?Sized + View> ViewCallbacks for T {
    default fn setup(&mut self) {}
    default fn update(&mut self) {}
}
