use gm::flat::{Point, Rect, Size};
use rtools::IntoF32;

use crate::{
    layout::{NewPlacer, Placer, Tiling},
    view::{ViewInternal, ViewSubviews},
    View,
};

pub trait ViewFrame {
    fn frame(&self) -> &Rect;
    fn super_frame(&self) -> &Rect;
    fn absolute_frame(&self) -> &Rect;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn max_x(&self) -> f32;
    fn max_y(&self) -> f32;
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn set_y(&mut self, y: impl IntoF32) -> &mut Self;
    fn set_origin(&mut self, origin: impl Into<Point>) -> &mut Self;
    fn set_center(&mut self, center: impl Into<Point>) -> &mut Self;
    fn set_frame(&mut self, rect: impl Into<Rect>) -> &mut Self;
    fn set_size(&mut self, size: impl Into<Size>) -> &mut Self;
    fn deprecated_place(&mut self) -> &mut Placer;
    fn calculate_frames(&mut self);
    fn new_layout(&mut self)
    where
        Self: View,
    {
        let view = self.view_mut();
        if let Some(placer) = &view.new_placer {
            placer.layout(&mut view.frame, view.superview.frame());
        }
        if let Some(tiling) = &view.tiling {
            tiling.layout(&mut view.frame, view.superview.frame(), &mut view.subviews);
        }
    }

    fn make_layout(&mut self, make: impl FnOnce(&mut NewPlacer)) -> &mut Self
    where
        Self: View,
    {
        debug_assert!(self.view_mut().tiling.is_none(), "Layout after tiling");
        debug_assert!(self.view_mut().new_placer.is_none(), "Double layout");
        let mut placer = NewPlacer::default();
        make(&mut placer);
        placer.assign_pending();
        self.view_mut().new_placer = placer.into();
        self
    }

    fn make_tiling(&mut self, make: impl FnOnce(&mut Tiling)) -> &mut Self
    where
        Self: View,
    {
        debug_assert!(self.view_mut().new_placer.is_none(), "Tiling after layout");
        debug_assert!(self.view_mut().tiling.is_none(), "Double tiling");
        let mut tiling = Tiling::default();
        make(&mut tiling);
        self.view_mut().tiling = tiling.into();
        self
    }
}

impl<T: ?Sized + View> ViewFrame for T {
    fn frame(&self) -> &Rect {
        &self.view().frame
    }

    fn super_frame(&self) -> &Rect {
        if self.view().superview.is_ok() {
            return self.view().superview.frame();
        }
        self.frame()
    }

    fn absolute_frame(&self) -> &Rect {
        &self.view().absolute_frame
    }

    fn x(&self) -> f32 {
        self.frame().origin.x
    }

    fn y(&self) -> f32 {
        self.frame().origin.y
    }

    fn max_x(&self) -> f32 {
        self.frame().max_x()
    }

    fn max_y(&self) -> f32 {
        self.frame().max_y()
    }

    fn width(&self) -> f32 {
        self.frame().size.width
    }

    fn height(&self) -> f32 {
        self.frame().size.height
    }

    fn set_y(&mut self, y: impl IntoF32) -> &mut Self {
        self.view_mut().frame.origin.y = y.into_f32();
        self
    }

    fn set_origin(&mut self, origin: impl Into<Point>) -> &mut Self {
        self.view_mut().frame.origin = origin.into();
        self
    }

    fn set_center(&mut self, center: impl Into<Point>) -> &mut Self {
        self.view_mut().frame.set_center(center.into());
        self
    }

    fn set_frame(&mut self, rect: impl Into<Rect>) -> &mut Self {
        self.view_mut().frame = rect.into();
        self
    }

    fn set_size(&mut self, size: impl Into<Size>) -> &mut Self {
        self.view_mut().frame.size = size.into();
        self
    }

    fn deprecated_place(&mut self) -> &mut Placer {
        &mut self.view_mut().placer
    }

    fn calculate_frames(&mut self) {
        self.layout();
        self.new_layout();
        let view = self.view_mut();
        view.absolute_frame = view.frame;
        view.absolute_frame.origin += view.super_absolute_frame().origin;
        for view in self.subviews_mut() {
            view.calculate_frames();
        }
    }
}
