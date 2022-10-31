use gm::flat::{Point, Rect, Size};
use rtools::IntoF32;

use crate::View;

pub trait ViewFrame {
    fn frame(&self) -> &Rect;
    fn super_frame(&self) -> &Rect;
    fn absolute_frame(&self) -> &Rect;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn max_x(&self) -> f32;
    fn max_y(&self) -> f32;
    fn size(&self) -> Size;
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn set_x(&mut self, x: impl IntoF32) -> &mut Self;
    fn set_y(&mut self, y: impl IntoF32) -> &mut Self;
    fn set_origin(&mut self, origin: impl Into<Point>) -> &mut Self;
    fn set_center(&mut self, center: impl Into<Point>) -> &mut Self;
    fn set_frame(&mut self, rect: impl Into<Rect>) -> &mut Self;
    fn set_size(&mut self, size: impl Into<Size>) -> &mut Self;
}

impl<T: ?Sized + View> ViewFrame for T {
    fn frame(&self) -> &Rect {
        &self.frame
    }

    fn super_frame(&self) -> &Rect {
        if self.superview.is_ok() {
            return self.superview.frame();
        }
        self.frame()
    }

    fn absolute_frame(&self) -> &Rect {
        &self.absolute_frame
    }

    fn size(&self) -> Size {
        self.frame.size
    }

    fn x(&self) -> f32 {
        self.frame.origin.x
    }

    fn y(&self) -> f32 {
        self.frame.origin.y
    }

    fn max_x(&self) -> f32 {
        self.frame.max_x()
    }

    fn max_y(&self) -> f32 {
        self.frame.max_y()
    }

    fn width(&self) -> f32 {
        self.frame.size.width
    }

    fn height(&self) -> f32 {
        self.frame.size.height
    }

    fn set_x(&mut self, x: impl IntoF32) -> &mut Self {
        self.frame.origin.x = x.into_f32();
        self
    }

    fn set_y(&mut self, y: impl IntoF32) -> &mut Self {
        self.frame.origin.y = y.into_f32();
        self
    }

    fn set_origin(&mut self, origin: impl Into<Point>) -> &mut Self {
        self.frame.origin = origin.into();
        self
    }

    fn set_center(&mut self, center: impl Into<Point>) -> &mut Self {
        self.frame.set_center(center.into());
        self
    }

    fn set_frame(&mut self, rect: impl Into<Rect>) -> &mut Self {
        self.frame = rect.into();
        self
    }

    fn set_size(&mut self, size: impl Into<Size>) -> &mut Self {
        self.frame.size = size.into();
        self
    }
}
