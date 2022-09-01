use crate::{
    layout::{NewPlacer, Placer, Tiling},
    view::view_internal::ViewInternal,
    View, ViewFrame, ViewSubviews,
};

pub trait ViewLayout {
    fn deprecated_place(&mut self) -> &mut Placer;
    fn calculate_frames(&mut self);
    fn new_layout(&mut self)
    where
        Self: View,
    {
        if self.superview().is_null() {
            return;
        }

        let view = self.view_mut();
        view.new_placer.layout(&mut view.frame, view.superview.frame());
        view.tiling
            .layout(&mut view.frame, view.superview.frame(), &mut view.subviews);
    }

    fn new_placer(&mut self) -> &mut NewPlacer
    where
        Self: View,
    {
        &mut self.view_mut().new_placer
    }

    fn tiling(&mut self) -> &mut Tiling
    where
        Self: View,
    {
        &mut self.view_mut().tiling
    }

    fn make_layout(&mut self, make: impl FnOnce(&mut NewPlacer)) -> &mut Self
    where
        Self: View,
    {
        make(self.new_placer());
        self
    }
}

impl<T: ?Sized + View> ViewLayout for T {
    fn deprecated_place(&mut self) -> &mut Placer {
        &mut self.view_mut().placer
    }

    fn calculate_frames(&mut self) {
        let view = self.view_mut();
        view.absolute_frame = view.frame;
        view.absolute_frame.origin += view.super_absolute_frame().origin;
        self.new_layout();
        self.layout();
        for view in self.subviews_mut() {
            view.calculate_frames();
        }
    }
}
