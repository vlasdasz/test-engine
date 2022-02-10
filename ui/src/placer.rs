use gm::Rect;
use rtools::{math::IntoF32, Rglica, ToRglica};

use crate::View;

#[derive(Default)]
pub struct Placer {
    view:    Rglica<dyn View>,
    frame:   Rglica<Rect>,
    s_frame: Rglica<Rect>,
}

impl Placer {
    pub fn make(view: &mut (dyn View + 'static)) -> Self {
        Self {
            view:    view.to_rglica(),
            frame:   view.frame().to_rglica(),
            s_frame: view.super_frame().to_rglica(),
        }
    }
}

impl Placer {
    pub fn as_background(&mut self) {
        self.frame.origin = Default::default();
        self.frame.size = self.s_frame.size;
    }

    pub fn center_hor(&mut self) {
        self.frame.origin.x = self.s_width() / 2.0 - self.width() / 2.0;
    }

    pub fn center_ver(&mut self) {
        self.frame.origin.y = self.s_height() / 2.0 - self.height() / 2.0;
    }

    pub fn center(&mut self) {
        self.center_hor();
        self.center_ver();
    }

    pub fn top_left_margin(&mut self, margin: impl IntoF32) {
        self.frame.origin.x = margin.into_f32();
        self.frame.origin.y = margin.into_f32();
    }

    pub fn top_right(&mut self) {
        self.frame.origin.x = self.s_width() - self.width();
        self.frame.origin.y = 0.0;
    }

    pub fn top_right_margin(&mut self, margin: impl IntoF32) {
        self.frame.origin.x = self.s_width() - self.width() - margin.into_f32();
        self.frame.origin.y = margin.into_f32();
    }

    pub fn bottom_left(&mut self) {
        self.frame.origin.x = 0.0;
        self.frame.origin.y = self.s_height() - self.height();
    }

    pub fn bottom_left_margin(&mut self, margin: impl IntoF32) {
        self.frame.origin.x = margin.into_f32();
        self.frame.origin.y = self.s_height() - self.height() - margin.into_f32();
    }

    pub fn bottom_right(&mut self) {
        self.frame.origin.x = self.s_width() - self.width();
        self.frame.origin.y = self.s_height() - self.height();
    }

    pub fn bottom_right_margin(&mut self, margin: impl IntoF32) {
        self.frame.origin.x = self.s_width() - self.width() - margin.into_f32();
        self.frame.origin.y = self.s_height() - self.height() - margin.into_f32();
    }

    pub fn left_half(&mut self) {
        *self.frame = (0, 0, self.s_width() / 2.0, self.s_height()).into();
    }

    pub fn right_half(&mut self) {
        let half_w = self.s_width() / 2.0;
        *self.frame = (half_w, 0, half_w, self.s_height()).into();
    }

    pub fn right(&mut self) {
        self.center_ver();
        self.frame.origin.x = self.s_width() - self.width();
    }

    pub fn at_center(&mut self, view: &dyn View) {
        self.frame.set_center(view.frame().center())
    }

    pub fn at_bottom(&mut self, view: &dyn View, margin: impl IntoF32) {
        self.at_center(view);
        self.frame.origin.y = view.frame().max_y() + margin.into_f32();
    }

    pub fn subviews_vertically(&mut self) {
        let views = self.view.subviews_mut();

        if views.is_empty() {
            return;
        }

        let mut frames: Vec<&mut Rect> = views.iter_mut().map(|a| a.frame_mut()).collect();
        let height: f32 = self.frame.height() / frames.len() as f32;
        let width = self.frame.width();

        for (i, frame) in frames.iter_mut().enumerate() {
            frame.origin.x = 0.0;
            frame.origin.y = i as f32 * height;
            frame.size.width = width;
            frame.size.height = height;
        }
    }
}

impl Placer {
    fn width(&self) -> f32 {
        self.frame.width()
    }

    fn height(&self) -> f32 {
        self.frame.height()
    }

    fn s_width(&self) -> f32 {
        self.s_frame.width()
    }

    fn s_height(&self) -> f32 {
        self.s_frame.height()
    }
}
