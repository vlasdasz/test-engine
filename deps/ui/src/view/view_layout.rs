use crate::{view::view_internal::ViewInternal, View};

pub trait ViewLayout {
    fn calculate_absolute_frame(&mut self);
    fn layout(&mut self)
    where Self: View {
        self.place.layout();
    }
}

impl<T: ?Sized + View> ViewLayout for T {
    fn calculate_absolute_frame(&mut self) {
        self.absolute_frame = self.frame;
        let orig = self.super_absolute_frame().origin;
        self.absolute_frame.origin += orig;
        let offset = self.content_offset;
        self.absolute_frame.origin += offset;
    }
}
