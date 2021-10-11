use std::default::default;

use gm::Rect;
use tools::{math::IntoF32, Rglica};

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
            view:    Rglica::from_ref(view),
            frame:   Rglica::from_ref(view.frame()),
            s_frame: Rglica::from_ref(view.super_frame()),
        }
    }
}

impl Placer {
    pub fn as_background(&mut self) {
        self.frame.origin = default();
        self.frame.size = self.s_frame.size;
    }

    pub fn center_in_superview(&mut self) {
        self.frame.set_center(self.s_frame.center());
    }

    pub fn top_left_margin(&mut self, margin: impl IntoF32) {
        self.frame.origin.x = margin.into_f32();
        self.frame.origin.y = margin.into_f32();
    }

    pub fn top_right(&mut self) {
        self.frame.origin.x = self.s_frame.size.width - self.frame.size.width;
        self.frame.origin.y = 0.0;
    }

    pub fn top_right_margin(&mut self, margin: impl IntoF32) {
        self.frame.origin.x = self.s_frame.size.width - self.frame.size.width - margin.into_f32();
        self.frame.origin.y = margin.into_f32();
    }

    pub fn bottom_left(&mut self) {
        self.frame.origin.x = 0.0;
        self.frame.origin.y = self.s_frame.size.height - self.frame.size.height;
    }

    pub fn bottom_left_margin(&mut self, margin: impl IntoF32) {
        self.frame.origin.x = margin.into_f32();
        self.frame.origin.y = self.s_frame.size.height - self.frame.size.height - margin.into_f32();
    }

    pub fn bottom_right(&mut self) {
        self.frame.origin.x = self.s_frame.size.width - self.frame.size.width;
        self.frame.origin.y = self.s_frame.size.height - self.frame.size.height;
    }

    pub fn bottom_right_margin(&mut self, margin: impl IntoF32) {
        self.frame.origin.x = self.s_frame.size.width - self.frame.size.width - margin.into_f32();
        self.frame.origin.y = self.s_frame.size.height - self.frame.size.height - margin.into_f32();
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
