use crate::{layout::Placer, view::view_internal::ViewInternal, View, ViewSubviews};

pub trait ViewLayout {
    fn calculate_frames(&mut self);
    fn layout(&mut self)
    where
        Self: View,
    {
        self.place().layout();
    }

    fn place(&mut self) -> &mut Placer
    where
        Self: View,
    {
        &mut self.view_mut().placer
    }

    fn make_layout(&mut self, make: impl FnOnce(&mut Placer)) -> &mut Self
    where
        Self: View,
    {
        make(self.place());
        self
    }
}

impl<T: ?Sized + View> ViewLayout for T {
    fn calculate_frames(&mut self) {
        let view = self.view_mut();
        view.absolute_frame = view.frame;
        view.absolute_frame.origin += view.super_absolute_frame().origin;
        self.layout();
        for view in self.subviews_mut() {
            view.calculate_frames();
        }
    }
}
