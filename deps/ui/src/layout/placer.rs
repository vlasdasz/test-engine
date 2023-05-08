use std::{
    cell::{RefCell, RefMut},
    ops::{Deref, DerefMut},
};

use gm::flat::{Rect, SizeBase};
use refs::{Own, Rglica, ToRglica, Weak};
use rtools::IntoF32;

use crate::{
    layout::{layout_rule::LayoutRule, Anchor, Tiling},
    view::ViewFrame,
    View, ViewSubviews,
};

pub struct Placer {
    pub(crate) rules:     RefCell<Vec<LayoutRule>>,
    pub(crate) sub_rules: RefCell<Vec<LayoutRule>>,

    view:        Weak<dyn View>,
    pub frame:   Rglica<Rect>,
    pub s_frame: Rglica<Rect>,

    has: RefCell<SizeBase<bool>>,
}

impl Placer {
    pub fn new(view: Weak<dyn View>) -> Self {
        Self {
            rules: vec![].into(),
            sub_rules: vec![].into(),
            view,
            frame: view.frame().to_rglica(),
            s_frame: view.super_frame().to_rglica(),
            has: Default::default(),
        }
    }

    pub fn rules_count(&self) -> usize {
        self.rules.borrow().len()
    }

    fn rules(&self) -> RefMut<Vec<LayoutRule>> {
        self.rules.borrow_mut()
    }

    fn sub_rules(&self) -> RefMut<Vec<LayoutRule>> {
        self.sub_rules.borrow_mut()
    }

    fn has(&self) -> RefMut<SizeBase<bool>> {
        self.has.borrow_mut()
    }
}

impl Placer {
    pub fn background(&self) -> &Self {
        self.t(0).b(0).l(0).r(0)
    }

    pub fn t(&self, offset: impl IntoF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::Top, offset.into_f32()));
        self
    }

    pub fn b(&self, offset: impl IntoF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::Bot, offset.into_f32()));
        self
    }

    pub fn l(&self, offset: impl IntoF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::Left, offset.into_f32()));
        self
    }

    pub fn r(&self, offset: impl IntoF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::Right, offset.into_f32()));
        self
    }

    pub fn size(&self, width: impl IntoF32, height: impl IntoF32) -> &Self {
        self.w(width).h(height)
    }

    pub fn same_size(&self, view: impl Deref<Target = impl View>) -> &Self {
        self.relative(Anchor::Size, 1, view)
    }

    pub fn same<const S: usize>(
        &self,
        anchors: [Anchor; S],
        view: impl Deref<Target = impl View> + Copy,
    ) -> &Self {
        for anchor in anchors {
            self.relative(anchor, 1, view);
        }
        self
    }

    pub fn w(&self, w: impl IntoF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::Width, w));
        self.has().width = true;
        self
    }

    pub fn h(&self, h: impl IntoF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::Height, h));
        self.has().height = true;
        self
    }

    pub fn center(&self) -> &Self {
        self.rules().push(Anchor::Center.into());
        self
    }

    pub fn center_hor(&self) -> &Self {
        self.rules().push(Anchor::CenterH.into());
        self
    }

    pub fn center_ver(&self) -> &Self {
        self.rules().push(Anchor::CenterV.into());
        self
    }

    pub fn as_background(&self) -> &Self {
        self.rules().push(Tiling::Background.into());
        self
    }

    pub fn all_ver(&self) -> &Self {
        self.sub_rules().push(Tiling::Vertically.into());
        self
    }

    pub fn all_hor(&self) -> &Self {
        self.sub_rules().push(Tiling::Horizontally.into());
        self
    }
}

impl Placer {
    pub fn max_width(&self, w: impl IntoF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::MaxWidth, w));
        self
    }

    pub fn max_height(&self, h: impl IntoF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::MaxHeight, h));
        self
    }
}

impl Placer {
    pub fn anchor(&self, view: impl Deref<Target = impl View>, side: Anchor, offset: impl IntoF32) -> &Self {
        self.rules().push(LayoutRule::anchor(side, offset, view.weak_view()));
        self
    }

    pub fn relative(&self, side: Anchor, ratio: impl IntoF32, view: impl Deref<Target = impl View>) -> &Self {
        self.has().width = if side.has_width() { true } else { self.has().width };
        self.has().height = if side.has_height() {
            true
        } else {
            self.has().height
        };

        self.rules().push(LayoutRule::relative(side, ratio, view.weak_view()));
        self
    }
}

impl Placer {
    pub fn lr(&self, offset: impl IntoF32) -> &Self {
        self.l(offset).r(offset)
    }

    pub fn tl(&self, offset: impl IntoF32) -> &Self {
        self.t(offset).l(offset)
    }

    pub fn tr(&self, offset: impl IntoF32) -> &Self {
        self.t(offset).r(offset)
    }

    pub fn bl(&self, offset: impl IntoF32) -> &Self {
        self.b(offset).l(offset)
    }

    pub fn br(&self, offset: impl IntoF32) -> &Self {
        self.b(offset).r(offset)
    }

    pub fn tlb(&self, offset: impl IntoF32) -> &Self {
        self.t(offset).l(offset).b(offset)
    }

    pub fn lrt(&self, offset: impl IntoF32) -> &Self {
        self.l(offset).r(offset).t(offset)
    }

    pub fn lrb(&self, offset: impl IntoF32) -> &Self {
        self.l(offset).r(offset).b(offset)
    }
}

impl Placer {
    pub fn above(&self, view: impl Deref<Target = impl View> + Copy, offset: impl IntoF32) -> &Self {
        self.same([Anchor::Size, Anchor::X], view).anchor(view, Anchor::Bot, offset)
    }

    pub fn between(
        &self,
        view_a: impl Deref<Target = impl View> + Copy,
        view_b: impl Deref<Target = impl View> + Copy,
    ) -> &Self {
        self.rules().push(LayoutRule::between(
            view_a.weak_view(),
            view_b.weak_view(),
            Anchor::None,
        ));
        self
    }

    pub fn between_super(&self, view: impl Deref<Target = impl View> + Copy, anchor: Anchor) -> &Self {
        self.rules()
            .push(LayoutRule::between(view.weak_view(), Default::default(), anchor));
        self
    }
}

impl Placer {
    pub fn layout(&mut self) {
        let this = self.to_rglica();

        for rule in this.rules().iter() {
            if rule.between {
                self.between_layout(rule);
            } else if rule.anchor_view.is_ok() {
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

        for rule in this.sub_rules().iter() {
            self.tiling_layout(rule.tiling.as_ref().expect("BUG"));
        }
    }
}

impl Placer {
    fn simple_layout(&mut self, rule: &LayoutRule) {
        let has = *self.has();
        let frame = self.frame.deref_mut();
        let s_frame = self.s_frame.deref();
        match rule.side {
            Anchor::Top => frame.origin.y = rule.offset,
            Anchor::Bot => {
                if has.height {
                    frame.origin.y = s_frame.height() - frame.height() - rule.offset
                } else {
                    frame.size.height = frame.height() + s_frame.height() - frame.max_y() - rule.offset
                }
            }
            Anchor::Left => frame.origin.x = rule.offset,
            Anchor::Right => {
                if has.width {
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
            Anchor::Left => frame.origin.x = a_frame.max_x() + rule.offset,
            Anchor::Right => frame.origin.x = a_frame.x() - rule.offset - frame.width(),
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
            Anchor::X => frame.origin.x = a_frame.origin.x * rule.offset,
            Anchor::Y => frame.origin.y = a_frame.origin.y * rule.offset,
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

    fn between_layout(&mut self, rule: &LayoutRule) {
        if rule.side.is_none() {
            self.between_2_layout(rule)
        } else {
            self.between_s_layout(rule);
        }
    }

    fn between_2_layout(&mut self, rule: &LayoutRule) {
        let center_a = rule.anchor_view.frame.center();
        let center_b = rule.anchor_view2.frame.center();
        let center = center_a.middle(&center_b);
        self.frame.set_center(center);
    }

    fn between_s_layout(&mut self, rule: &LayoutRule) {
        let f = rule.anchor_view.frame();
        let cen = f.center();
        match rule.side {
            Anchor::Top => self.frame.set_center((cen.x, f.y() / 2.0)),
            Anchor::Bot => self.frame.set_center((
                cen.x,
                self.s_frame.height() - (self.s_frame.height() - f.max_y()) / 2.0,
            )),
            Anchor::Left => self.frame.set_center((f.x() / 2.0, cen.y)),
            Anchor::Right => self.frame.set_center((
                self.s_frame.width() - (self.s_frame.width() - f.max_x()) / 2.0,
                cen.y,
            )),
            _ => unimplemented!(),
        }
    }
}

fn place_vertically(views: &mut [Own<dyn View>]) {
    if views.is_empty() {
        return;
    }

    let mut last = views.last_mut().unwrap().weak_view();

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
    Arr: AsMut<[Ref]>, {
    let views = views.as_mut();

    if views.is_empty() {
        return;
    }

    let mut last = views.last_mut().unwrap().weak_view();

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
