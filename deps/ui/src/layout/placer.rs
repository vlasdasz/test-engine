use std::ops::{Deref, DerefMut};

use gm::flat::Rect;
use rtools::{weak::ToWeak, IntoF32, Rglica};

use crate::{
    layout::{layout_rule::LayoutRule, Anchor, Tiling},
    view::ViewFrame,
    View, ViewSubviews,
};

pub struct Placer {
    pub(crate) rules: Vec<LayoutRule>,

    view:    Rglica<dyn View>,
    frame:   Rglica<Rect>,
    s_frame: Rglica<Rect>,

    has_width:  bool,
    has_height: bool,
}

impl Placer {
    pub fn new(view: Rglica<dyn View>) -> Self {
        Self {
            rules: vec![],
            view,
            frame: view.frame().weak(),
            s_frame: view.super_frame().weak(),
            has_width: false,
            has_height: false,
        }
    }

    pub fn rules_count(&self) -> usize {
        self.rules.len()
    }
}

impl Placer {
    pub fn background(&mut self) -> &mut Self {
        self.t(0).b(0).l(0).r(0)
    }

    pub fn t(&mut self, offset: impl IntoF32) -> &mut Self {
        self.rules.push(LayoutRule::make(Anchor::Top, offset.into_f32()));
        self
    }

    pub fn b(&mut self, offset: impl IntoF32) -> &mut Self {
        self.rules.push(LayoutRule::make(Anchor::Bot, offset.into_f32()));
        self
    }

    pub fn l(&mut self, offset: impl IntoF32) -> &mut Self {
        self.rules.push(LayoutRule::make(Anchor::Left, offset.into_f32()));

        self
    }

    pub fn r(&mut self, offset: impl IntoF32) -> &mut Self {
        self.rules.push(LayoutRule::make(Anchor::Right, offset.into_f32()));
        self
    }

    pub fn size(&mut self, width: impl IntoF32, height: impl IntoF32) -> &mut Self {
        self.w(width).h(height)
    }

    pub fn w(&mut self, w: impl IntoF32) -> &mut Self {
        self.rules.push(LayoutRule::make(Anchor::Width, w));
        self.has_width = true;
        self
    }

    pub fn h(&mut self, h: impl IntoF32) -> &mut Self {
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
}

impl Placer {
    pub fn max_width(&mut self, w: impl IntoF32) -> &mut Self {
        self.rules.push(LayoutRule::make(Anchor::MaxWidth, w));
        self
    }

    pub fn max_height(&mut self, h: impl IntoF32) -> &mut Self {
        self.rules.push(LayoutRule::make(Anchor::MaxHeight, h));
        self
    }
}

impl Placer {
    pub fn anchor(&mut self, view: impl Deref<Target = impl View>, side: Anchor, offset: impl IntoF32) {
        self.rules.push(LayoutRule::anchor(side, offset, view.weak_view()));
    }

    pub fn relative(&mut self, view: impl Deref<Target = impl View>, side: Anchor, ratio: impl IntoF32) {
        self.rules.push(LayoutRule::relative(side, ratio, view.weak_view()))
    }
}

impl Placer {
    pub fn lr(&mut self, offset: impl IntoF32) -> &mut Self {
        self.l(offset).r(offset)
    }

    pub fn tl(&mut self, offset: impl IntoF32) -> &mut Self {
        self.t(offset).l(offset)
    }

    pub fn tr(&mut self, offset: impl IntoF32) -> &mut Self {
        self.t(offset).r(offset)
    }

    pub fn bl(&mut self, offset: impl IntoF32) -> &mut Self {
        self.b(offset).l(offset)
    }

    pub fn br(&mut self, offset: impl IntoF32) -> &mut Self {
        self.b(offset).r(offset)
    }

    pub fn tlb(&mut self, offset: impl IntoF32) -> &mut Self {
        self.t(offset).l(offset).b(offset)
    }

    pub fn lrt(&mut self, offset: impl IntoF32) -> &mut Self {
        self.l(offset).r(offset).t(offset)
    }

    pub fn lrb(&mut self, offset: impl IntoF32) -> &mut Self {
        self.l(offset).r(offset).b(offset)
    }
}

impl Placer {
    pub fn layout(&mut self) {
        let this = self.weak();
        for rule in &this.rules {
            if rule.anchor_view.is_ok() {
                if rule.relative {
                    self.relative_layout(rule)
                } else {
                    self.anchor_layout(rule)
                }
            } else if let Some(tiling) = &rule.tiling {
                self.tiling_layout(tiling);
            } else {
                self.simple_layout(rule)
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
            Anchor::MaxWidth => {
                if frame.size.width > rule.offset {
                    frame.size.width = rule.offset
                }
            }
            Anchor::MaxHeight => {
                if frame.size.height > rule.offset {
                    frame.size.height = rule.offset
                }
            }
            _ => unimplemented!(),
        }
    }

    fn anchor_layout(&mut self, rule: &LayoutRule) {
        let frame = self.frame.deref_mut();
        let a_frame = rule.anchor_view.frame();
        match rule.side {
            Anchor::Top => frame.origin.y = a_frame.max_y() + rule.offset,
            Anchor::Bot => frame.origin.y = a_frame.y() - rule.offset - frame.height(),
            _ => unimplemented!(),
        }
    }

    fn relative_layout(&mut self, rule: &LayoutRule) {
        let frame = self.frame.deref_mut();
        let a_frame = rule.anchor_view.frame();
        match rule.side {
            Anchor::Width => frame.size.width = a_frame.size.width * rule.offset,
            Anchor::Height => frame.size.height = a_frame.size.height * rule.offset,
            Anchor::Size => frame.size = a_frame.size * rule.offset,
            _ => unimplemented!(),
        }
    }

    fn tiling_layout(&mut self, tiling: &Tiling) {
        let mut frame = self.frame;
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

    let mut last = views.last_mut().unwrap().weak();

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

    let mut last = views.last_mut().unwrap().weak();

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
