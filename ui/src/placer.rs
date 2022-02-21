use std::ops::{Deref, DerefMut};

use gm::Rect;
use rtools::{math::IntoF32, Rglica, ToRglica};

use crate::View;

pub enum Anchor {
    Top,
    Bot,

    Left,
    Right,

    Center,
}

impl Anchor {
    pub fn is_vertical(&self) -> bool {
        matches!(self, Anchor::Top | Anchor::Bot)
    }
    pub fn is_horizontal(&self) -> bool {
        matches!(self, Anchor::Left | Anchor::Right)
    }
}

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
        *self.frame = self.s_frame.with_zero_origin();
    }

    pub fn background_margin(&mut self, margin: impl IntoF32) {
        let margin = margin.into_f32();
        self.frame.origin = (margin, margin).into();
        self.frame.size = (self.s_width() - margin * 2.0, self.s_height() - margin * 2.0).into();
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

    pub fn top_left(&mut self, margin: impl IntoF32) {
        self.frame.origin.x = margin.into_f32();
        self.frame.origin.y = margin.into_f32();
    }

    pub fn top_right(&mut self, margin: impl IntoF32) {
        self.frame.origin.x = self.s_width() - self.width() - margin.into_f32();
        self.frame.origin.y = margin.into_f32();
    }

    pub fn bottom_left(&mut self, margin: impl IntoF32) {
        self.frame.origin.x = margin.into_f32();
        self.frame.origin.y = self.s_height() - self.height() - margin.into_f32();
    }

    pub fn bottom_right(&mut self, margin: impl IntoF32) {
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

    pub fn at_right(&mut self, view: &dyn View, margin: impl IntoF32) {
        self.at_center(view);
        self.frame.origin.x = view.frame().max_x() + margin.into_f32();
    }

    pub fn all_vertically(&mut self) {
        place_vertically(self.view.subviews_mut());
    }

    pub fn all_vertically_with_ratio<const N: usize>(&mut self, ratio: [impl IntoF32; N]) {
        if self.subviews().len() != ratio.len() {
            panic!("Invalid ratio len");
        }

        let total_ratio: f32 = ratio.iter().map(|a| a.into_f32()).sum();
        let total_ratio = 1.0 / total_ratio;

        let mut subs: Vec<_> = self.subviews_mut().iter().map(|a| a.to_rglica()).collect();

        for (i, view) in subs.iter_mut().enumerate() {
            let is_first = i == 0;
            let prev_index = if is_first { 0 } else { i - 1 };
            let y_pos = if is_first {
                0.0
            } else {
                self.subviews()[prev_index].frame().max_y()
            };
            view.set_frame(
                (
                    0,
                    y_pos,
                    self.width(),
                    ratio[i].into_f32() * self.height() * total_ratio,
                )
                    .into(),
            );
        }
    }

    pub fn anchor(
        &mut self,
        view: impl Deref<Target = impl View + ?Sized>,
        anchor: Anchor,
        position: Anchor,
        margin: impl IntoF32,
    ) {
        let margin = margin.into_f32();

        match anchor {
            Anchor::Top => {
                self.frame.origin.y = view.y() - margin - self.height();
            }
            Anchor::Bot => {
                self.frame.origin.y = view.max_y() + margin;
            }
            Anchor::Left => {
                self.frame.origin.x = view.x() - margin - self.width();
            }
            Anchor::Right => {
                self.frame.origin.x = view.max_x() + margin;
            }
            Anchor::Center => {
                self.frame.origin.x = view.x() - view.height() / 2.0 + self.height() / 2.0;
                self.frame.origin.y = view.y() - view.width() / 2.0 + self.width() / 2.0;
            }
        }

        match position {
            Anchor::Top => {
                self.frame.origin.y = view.y();
            }
            Anchor::Bot => {
                self.frame.origin.y = view.max_y() - self.height();
            }
            Anchor::Left => {
                self.frame.origin.x = view.x();
            }
            Anchor::Right => {
                self.frame.origin.x = view.max_x() - self.width();
            }
            Anchor::Center => {
                if anchor.is_horizontal() {
                    self.frame.origin.y = view.y() + view.height() / 2.0 - self.height() / 2.0;
                } else {
                    self.frame.origin.x = view.x() + view.width() / 2.0 - self.width() / 2.0;
                }
            }
        }
    }
}

impl Placer {
    pub fn proportional_width(&mut self, ratio: impl IntoF32) {
        self.frame.size.width = self.s_width() * ratio.into_f32()
    }

    pub fn proportional_height(&mut self, ratio: impl IntoF32) {
        self.frame.size.height = self.s_height() * ratio.into_f32()
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

    fn subviews(&self) -> &[Box<dyn View>] {
        self.view.subviews()
    }

    fn subviews_mut(&mut self) -> &mut [Box<dyn View>] {
        self.view.subviews_mut()
    }
}

pub fn place_vertically<T, Ref, Arr>(mut views: Arr)
where
    T: View + ?Sized,
    Ref: DerefMut<Target = T>,
    Arr: AsMut<[Ref]>,
{
    let views = views.as_mut();

    if views.is_empty() {
        return;
    }

    let mut last = views.last_mut().unwrap().to_rglica();

    if views.len() == 1 {
        last.place().as_background();
        return;
    }

    let super_frame = *last.superview().frame();

    let height = super_frame.height() / views.len() as f32;
    let width = super_frame.width();

    for (i, frame) in views.iter_mut().map(|a| a.frame_mut()).enumerate() {
        frame.origin.x = 0.0;
        frame.origin.y = i as f32 * height;
        frame.size.width = width;
        frame.size.height = height;
    }
}
