use gm::flat::{Point, Rect};
use rtools::IntoF32;

use crate::{placer::Placer, View};

pub trait ViewFrame {
    fn frame(&self) -> &Rect;
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
    fn place(&mut self) -> &mut Placer;
}

impl<T: ?Sized + View> ViewFrame for T {
    fn frame(&self) -> &Rect {
        &self.view().frame
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

    fn place(&mut self) -> &mut Placer {
        &mut self.view_mut().placer
    }
}
