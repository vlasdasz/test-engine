use crate::{
    view::{view_frame::ViewFrame, view_internal::ViewInternal},
    View,
};

pub trait ViewLayout {
    fn calculate_absolute_frame(&mut self);
    fn layout(&mut self)
    where Self: View {
        self.base_mut().placer.layout();
    }
}

impl<T: ?Sized + View> ViewLayout for T {
    default fn calculate_absolute_frame(&mut self) {
        self.base_mut().absolute_frame = *self.frame();
        let orig = self.super_absolute_frame().origin;
        self.base_mut().absolute_frame.origin += orig;
    }
}
