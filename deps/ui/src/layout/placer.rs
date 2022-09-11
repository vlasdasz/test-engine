use std::ops::{Deref, DerefMut};

use gm::flat::Rect;
use rtools::{IntoF32, Rglica, ToRglica};

use crate::{
    layout::{layout_rule::LayoutRule, Anchor, Tiling},
    view::ViewFrame,
    View, ViewSubviews,
};

#[derive(Default)]
pub struct Placer {
    pub(crate) rules: Vec<LayoutRule>,
    pending_sides:    Vec<Anchor>,

    view:    Rglica<dyn View>,
    frame:   Rglica<Rect>,
    s_frame: Rglica<Rect>,

    has_width:  bool,
    has_height: bool,
}

impl Placer {
    pub fn make(view: Rglica<dyn View>) -> Self {
        Self {
            view,
            frame: view.frame().to_rglica(),
            s_frame: view.super_frame().to_rglica(),
            ..Default::default()
        }
    }
}

impl Placer {
    pub fn background(&mut self) -> &mut Self {
        self.top().bottom().left().right()
    }

    pub fn top(&mut self) -> &mut Self {
        self.pending_sides.push(Anchor::Top);
        self
    }

    pub fn bottom(&mut self) -> &mut Self {
        self.pending_sides.push(Anchor::Bot);
        self
    }

    pub fn left(&mut self) -> &mut Self {
        self.pending_sides.push(Anchor::Left);
        self
    }

    pub fn right(&mut self) -> &mut Self {
        self.pending_sides.push(Anchor::Right);
        self
    }

    pub fn size(&mut self, width: impl IntoF32, height: impl IntoF32) -> &mut Self {
        self.width(width).height(height)
    }

    pub fn width(&mut self, w: impl IntoF32) -> &mut Self {
        self.rules.push(LayoutRule::make(Anchor::Width, w));
        self.has_width = true;
        self
    }

    pub fn height(&mut self, h: impl IntoF32) -> &mut Self {
        self.rules.push(LayoutRule::make(Anchor::Height, h));
        self.has_height = true;
        self
    }

    pub fn center(&mut self) -> &mut Self {
        self.rules.push(Anchor::Center.into());
        self
    }

    pub fn center_hor(&mut self) -> &mut Self {
        self.rules.push(Anchor::CenterH.into());
        self
    }

    pub fn center_ver(&mut self) -> &mut Self {
        self.rules.push(Anchor::CenterV.into());
        self
    }

    pub fn as_background(&mut self) -> &mut Self {
        self.rules.push(Tiling::Background.into());
        self
    }

    pub fn all_ver(&mut self) -> &mut Self {
        self.rules.push(Tiling::Vertically.into());
        self
    }

    pub fn all_hor(&mut self) -> &mut Self {
        self.rules.push(Tiling::Horizontally.into());
        self
    }

    pub fn val(&mut self, offset: impl IntoF32) -> &mut Self {
        let pending = self.pending_sides.drain(..);
        self.rules
            .extend(pending.map(|a| LayoutRule::make(a, offset.into_f32())));
        self
    }

    pub fn anchor<T: View>(&mut self, view: Rglica<T>, offset: impl IntoF32) {
        debug_assert!(
            self.pending_sides.len() == 1,
            "Anchor should be to exactly one size"
        );
        let side = self.pending_sides.pop().unwrap();
        self.rules.push(LayoutRule::anchor(side, offset, view.rglica()));
    }

    fn assign_pending(&mut self) {
        let pending = self.pending_sides.drain(..);
        self.rules.extend(pending.map(|a| a.into()))
    }
}

impl Placer {
    pub fn tl(&mut self) -> &mut Self {
        self.top().left()
    }

    pub fn tr(&mut self) -> &mut Self {
        self.top().right()
    }

    pub fn bl(&mut self) -> &mut Self {
        self.bottom().left()
    }

    pub fn br(&mut self) -> &mut Self {
        self.bottom().right()
    }
}

impl Placer {
    pub fn layout(&mut self) {
        self.assign_pending();
        let this = self.to_rglica();
        for rule in &this.rules {
            if rule.anchor_view.is_ok() {
                self.anchor_layout(rule)
            } else {
                if let Some(tiling) = &rule.tiling {
                    self.tiling_layout(tiling);
                } else {
                    self.simple_layout(rule)
                }
            }
        }
    }
}

impl Placer {
    fn simple_layout(&mut self, rule: &LayoutRule) {
        let frame = self.frame.deref_mut();
        let s_frame = self.s_frame.deref();
        match rule.side {
            Anchor::Top => frame.origin.y = rule.offset,
            Anchor::Bot => {
                if self.has_height {
                    frame.origin.y = s_frame.height() - frame.height() - rule.offset
                } else {
                    frame.size.height = frame.height() + s_frame.height() - frame.max_y() - rule.offset
                }
            }
            Anchor::Left => frame.origin.x = rule.offset,
            Anchor::Right => {
                if self.has_width {
                    frame.origin.x = s_frame.width() - frame.width() - rule.offset;
                } else {
                    frame.size.width = frame.width() + s_frame.width() - frame.max_x() - rule.offset
                }
            }
            Anchor::Width => frame.size.width = rule.offset,
            Anchor::Height => frame.size.height = rule.offset,
            Anchor::CenterH => frame.origin.x = s_frame.width() / 2.0 - frame.width() / 2.0,
            Anchor::CenterV => frame.origin.y = s_frame.height() / 2.0 - frame.height() / 2.0,
            Anchor::Center => {
                frame.origin.x = s_frame.width() / 2.0 - frame.width() / 2.0;
                frame.origin.y = s_frame.height() / 2.0 - frame.height() / 2.0;
            }
        }
    }

    fn anchor_layout(&mut self, rule: &LayoutRule) {
        let frame = self.frame.deref_mut();
        let a_frame = rule.anchor_view.frame();
        match rule.side {
            Anchor::Top => frame.origin.y = a_frame.max_y() + rule.offset,
            _ => unimplemented!(),
        }
    }

    fn tiling_layout(&mut self, tiling: &Tiling) {
        let mut frame = self.frame.clone();
        let frame = frame.deref_mut();
        match tiling {
            Tiling::Background => *frame = self.s_frame.with_zero_origin(),
            Tiling::Horizontally => place_horizontally(self.view.subviews_mut()),
            Tiling::Vertically => place_vertically(self.view.subviews_mut()),
        }
    }
}

fn place_vertically<T, Ref, Arr>(mut views: Arr)
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
        let back = last.super_frame().with_zero_origin();
        last.set_frame(back);
        return;
    }

    let super_frame = *last.superview().frame();

    let height = super_frame.height() / views.len() as f32;
    let width = super_frame.width();

    for (i, view) in views.iter_mut().enumerate() {
        view.set_frame((0.0, i as f32 * height, width, height));
    }
}

fn place_horizontally<T, Ref, Arr>(mut views: Arr)
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
        let back = last.super_frame().with_zero_origin();
        last.set_frame(back);
        return;
    }

    let super_frame = *last.superview().frame();

    let width = super_frame.width() / views.len() as f32;
    let height = super_frame.height();

    for (i, view) in views.iter_mut().enumerate() {
        view.set_frame((i as f32 * width, 0, width, height));
    }
}
