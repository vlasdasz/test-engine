use crate::{
    View,
    view::{view_frame::ViewFrame, view_internal::ViewInternal},
};

pub trait ViewLayout {
    fn calculate_absolute_frame(&mut self);
    fn layout(&mut self)
    where Self: View {
        self.base_view().placer.layout();
    }
}

impl<T: ?Sized + View> ViewLayout for T {
    fn calculate_absolute_frame(&mut self) {
        self.base_view().absolute_frame = *self.frame();
        let orig = self.super_absolute_frame().origin;
        self.base_view().absolute_frame.origin += orig;
        let offset = self.base_view().content_offset;
        self.base_view().absolute_frame.origin.y += offset;
    }
}
