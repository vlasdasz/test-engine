use crate::{
    view::{view_data::ViewData, view_frame::ViewFrame, view_internal::ViewInternal},
    View,
};

pub trait ViewLayout {
    fn calculate_absolute_frame(&mut self);
    fn layout(&mut self)
    where Self: View {
        self.place().layout();
    }
}

impl<T: ?Sized + View> ViewLayout for T {
    fn calculate_absolute_frame(&mut self) {
        self.base_mut().absolute_frame = *self.frame();
        let orig = self.super_absolute_frame().origin;
        self.base_mut().absolute_frame.origin += orig;
        let offset = self.base().content_offset;
        self.base_mut().absolute_frame.origin += offset;
    }
}
