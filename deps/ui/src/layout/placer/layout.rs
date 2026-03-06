use std::ops::Deref;

use gm::{
    LossyConvert,
    axis::Axis,
    flat::{Rect, Size},
};
use refs::ToRglica;

use super::Placer;
use crate::{
    ViewSubviews, WeakView,
    layout::{Anchor, Tiling, layout_rule::LayoutRule},
    view::ViewFrame,
};

type RMut<'a> = &'a mut Rect;

impl Placer {
    pub fn layout(&mut self) {
        if self.view.is_null() {
            return;
        }

        let this = self.to_rglica();

        let has_left = self.has_left();
        let has_top = self.has_top();

        let old_frame = *self.view.frame();
        let mut new_frame = old_frame;

        let s_content = self.s_content.deref().clone();

        for rule in this.rules().iter_mut().filter(|r| r.enabled) {
            if rule.between {
                self.between_layout(&mut new_frame, rule);
            } else if rule.anchor_view.is_some() {
                if rule.relative {
                    self.relative_layout(&mut new_frame, rule);
                } else if rule.same {
                    same_layout(&mut new_frame, rule);
                } else {
                    anchor_layout(&mut new_frame, rule, has_left, has_top);
                }
            } else if let Some(tiling) = &rule.tiling {
                self.tiling_layout(&mut new_frame, &s_content, tiling);
            } else {
                self.simple_layout(&mut new_frame, rule);
            }
        }

        for rule in this.all_tiling_rules().iter().filter(|r| r.enabled) {
            self.tiling_layout(&mut new_frame, &s_content, rule.tiling.as_ref().expect("BUG"));
        }

        if let Some(custom) = self.custom.borrow().as_ref() {
            custom.lock()(&mut new_frame);
        }

        if new_frame != old_frame {
            self.view.set_frame(new_frame);
        }
    }

    fn simple_layout(&mut self, frame: RMut, rule: &LayoutRule) {
        let has = *self.has();
        let s_content = self.s_content.deref();

        let side = rule.side.as_ref().expect("Reached side layout with no side rule");

        match side {
            Anchor::Top => {
                if !has.height {
                    frame.size.height = frame.max_y() - rule.offset;
                }

                frame.origin.y = rule.offset;
            }
            Anchor::Bot => {
                if has.height {
                    frame.origin.y = s_content.height - frame.height() - rule.offset;
                } else {
                    frame.size.height = frame.height() + s_content.height - frame.max_y() - rule.offset;
                }
            }
            Anchor::Left => {
                if !has.width {
                    frame.size.width = frame.max_x() - rule.offset;
                }

                frame.origin.x = rule.offset;
            }
            Anchor::Right => {
                if has.width {
                    frame.origin.x = s_content.width - frame.width() - rule.offset;
                } else {
                    frame.size.width = s_content.width - frame.origin.x - rule.offset;
                }
            }
            Anchor::Width => frame.size.width = rule.offset,
            Anchor::Height => frame.size.height = rule.offset,
            Anchor::CenterX => frame.origin.x = s_content.width / 2.0 - frame.width() / 2.0,
            Anchor::CenterY => frame.origin.y = s_content.height / 2.0 - frame.height() / 2.0,
            Anchor::Center => {
                frame.origin.x = s_content.width / 2.0 - frame.width() / 2.0;
                frame.origin.y = s_content.height / 2.0 - frame.height() / 2.0;
            }
            Anchor::MaxWidth => {
                if frame.size.width > rule.offset {
                    frame.size.width = rule.offset;
                }
            }
            Anchor::MaxHeight => {
                if frame.size.height > rule.offset {
                    frame.size.height = rule.offset;
                }
            }
            Anchor::MinWidth => {
                if frame.size.width < rule.offset {
                    frame.size.width = rule.offset;
                }
            }
            Anchor::MinHeight => {
                if frame.size.height < rule.offset {
                    frame.size.height = rule.offset;
                }
            }
            Anchor::X | Anchor::Y | Anchor::None => {
                unimplemented!("Simple layout for {:?} is not supported", side)
            }
        }
    }

    fn relative_layout(&mut self, frame: RMut, rule: &LayoutRule) {
        let a_frame = rule.anchor_view.as_ref().expect("No anchor view in relative layout").frame();

        let side = rule.side.as_ref().expect("Relative layout without side");

        match side {
            Anchor::Width => frame.size.width = a_frame.size.width * rule.offset,
            Anchor::Height => frame.size.height = a_frame.size.height * rule.offset,
            Anchor::X => frame.origin.x = a_frame.width() * rule.offset,
            Anchor::Y => frame.origin.y = a_frame.height() * rule.offset,
            Anchor::CenterY => {
                let s_content = self.s_content.deref();
                let mut center = s_content.center();
                center.y += rule.offset;
                frame.set_center(center);
            }
            _ => unimplemented!("Relative layout for {:?} is not supported", side),
        }
    }

    fn tiling_layout(&mut self, frame: RMut, s_content: &Size, tiling: &Tiling) {
        match tiling {
            Tiling::Background => *frame = s_content.clone().into(),
            Tiling::Horizontally => place_horizontally(self.view.subviews_mut(), *self.all_margin.borrow()),
            Tiling::Vertically => place_vertically(self.view.subviews_mut(), *self.all_margin.borrow()),
            Tiling::LeftHalf => *frame = (0, 0, s_content.width / 2.0, s_content.height).into(),
            Tiling::RightHalf => {
                *frame = (s_content.width / 2.0, 0, s_content.width / 2.0, s_content.height).into();
            }
            Tiling::Distribute(ratio) => distribute_with_ratio(frame.size, self.view.subviews_mut(), ratio),
        }
    }

    fn between_layout(&mut self, frame: RMut, rule: &LayoutRule) {
        if rule.side.is_none() {
            between_2_layout(frame, rule);
        } else {
            self.between_super_layout(frame, rule);
        }
    }

    fn between_super_layout(&mut self, frame: RMut, rule: &LayoutRule) {
        let f = rule
            .anchor_view
            .as_ref()
            .expect("No anchor view in between super layout")
            .frame();
        let cen = f.center();

        let side = rule.side.as_ref().expect("Between layout without side");

        match side {
            Anchor::Top => frame.set_center((cen.x, f.y() / 2.0)),
            Anchor::Bot => frame.set_center((
                cen.x,
                self.s_content.height - (self.s_content.height - f.max_y()) / 2.0,
            )),
            Anchor::Left => frame.set_center((f.x() / 2.0, cen.y)),
            Anchor::Right => frame.set_center((
                self.s_content.width - (self.s_content.width - f.max_x()) / 2.0,
                cen.y,
            )),
            _ => unimplemented!("Between super layout for {:?} is not supported", side),
        }
    }

    fn has_left(&self) -> bool {
        self.rules.borrow().iter().any(|rule| rule.side.is_some_and(Anchor::is_left))
    }

    fn has_top(&self) -> bool {
        self.rules.borrow().iter().any(|rule| rule.side.is_some_and(Anchor::is_top))
    }
}

fn place_vertically(views: Vec<WeakView>, margin: f32) {
    distribute::<{ Axis::Y }>(views, margin);
}

fn place_horizontally(views: Vec<WeakView>, margin: f32) {
    distribute::<{ Axis::X }>(views, margin);
}

fn distribute<const AXIS: Axis>(mut views: Vec<WeakView>, margin: f32) {
    let Some(last) = views.last_mut().map(|v| v.weak_view()) else {
        return;
    };

    let super_frame = *last.superview().frame();

    if views.len() == 1 {
        let back = super_frame.with_zero_origin();
        last.set_frame(back);
        return;
    }

    let all_margins = margin * (views.len() - 1).lossy_convert();

    let left_length = super_frame.length::<AXIS>() - all_margins;

    let length = left_length / views.len().lossy_convert();
    let other_length = super_frame.other_length::<AXIS>();

    let mut last_pos: f32 = 0.0;

    for view in &mut views {
        let mut frame = *view.frame();

        frame.set_position::<AXIS>(last_pos);
        frame.set_other_position::<AXIS>(0);
        frame.set_length::<AXIS>(length);
        frame.set_other_length::<AXIS>(other_length);

        view.set_frame(frame);

        last_pos += length + margin;
    }
}

fn distribute_with_ratio(size: Size, views: Vec<WeakView>, ratios: &[f32]) {
    let total_ratio = 1.0 / ratios.iter().sum::<f32>();

    for i in 0..ratios.len() {
        let is_first = i == 0;
        let x_pos = if is_first { 0.0 } else { views[i - 1].max_x() };
        views[i].set_frame((x_pos, 0, ratios[i] * size.width * total_ratio, size.height));
    }
}

fn anchor_layout(frame: RMut, rule: &LayoutRule, has_left: bool, has_top: bool) {
    let a_frame = rule.anchor_view.as_ref().expect("No anchor view in anchor layout").frame();

    let side = rule.side.as_ref().expect("Anchor layout without side");

    match side {
        Anchor::Top => frame.origin.y = a_frame.max_y() + rule.offset,
        Anchor::Bot => {
            if has_top {
                let max_y = frame.max_y();
                let desired_max_y = a_frame.y() - rule.offset;
                let diff = desired_max_y - max_y;
                frame.size.height += diff;
            } else {
                frame.origin.y = a_frame.y() - rule.offset - frame.height();
            }
        }
        Anchor::Left => frame.origin.x = a_frame.max_x() + rule.offset,
        Anchor::Right => {
            if has_left {
                let max_x = frame.max_x();
                let desired_max_x = a_frame.x() - rule.offset;
                let diff = desired_max_x - max_x;
                frame.size.width += diff;
            } else {
                frame.origin.x = a_frame.x() - rule.offset - frame.width();
            }
        }
        Anchor::X => frame.origin.x = a_frame.x(),
        Anchor::Y => frame.origin.y = a_frame.y(),
        Anchor::Width => frame.size.width = a_frame.width(),
        Anchor::Height => frame.size.height = a_frame.height(),
        _ => unimplemented!("Anchor layout for: {:?} is not supported", side),
    }
}

fn between_2_layout(frame: RMut, rule: &LayoutRule) {
    let center_a = rule.anchor_view.expect("No anchor view in between 2 layout").frame().center();
    let center_b = rule
        .anchor_view2
        .expect("No anchor 2 view in between 2 layout")
        .frame()
        .center();
    let center = center_a.middle(&center_b);
    frame.set_center(center);
}

fn same_layout(frame: RMut, rule: &LayoutRule) {
    let a_frame = rule.anchor_view.as_ref().expect("No anchor view in same layout").frame();

    let side = rule.side.as_ref().expect("Same layout without side");

    match side {
        Anchor::Width => frame.size.width = a_frame.size.width,
        Anchor::Height => frame.size.height = a_frame.size.height,
        Anchor::X => frame.origin.x = a_frame.x(),
        Anchor::Y => frame.origin.y = a_frame.y(),
        Anchor::CenterX => {
            let mut frame_center = frame.center();
            let a_center = a_frame.center();
            frame_center.x = a_center.x;
            frame.set_center(frame_center);
        }
        Anchor::CenterY => {
            let mut frame_center = frame.center();
            let a_center = a_frame.center();
            frame_center.y = a_center.y;
            frame.set_center(frame_center);
        }
        _ => unimplemented!("Same layout for {:?} is not supported", side),
    }
}
