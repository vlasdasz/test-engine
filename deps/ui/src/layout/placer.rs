use std::{
    cell::{RefCell, RefMut},
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
};

use gm::{
    axis::Axis,
    flat::{Rect, Size, SizeBase},
};
use itertools::Itertools;
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

    view:      Weak<dyn View>,
    frame:     Rglica<Rect>,
    s_content: Rglica<Size>,

    all_margin: RefCell<f32>,

    has: RefCell<SizeBase<bool>>,
}

impl Placer {
    pub fn empty() -> Self {
        Self {
            rules:      RefCell::new(vec![]),
            sub_rules:  RefCell::new(vec![]),
            view:       Default::default(),
            frame:      Default::default(),
            s_content:  Default::default(),
            all_margin: RefCell::new(0.0),
            has:        RefCell::new(Default::default()),
        }
    }

    pub fn new(view: Weak<dyn View>) -> Self {
        let s_content = if view.superview.is_ok() {
            view.superview.content_size()
        } else {
            view.content_size()
        };

        Self {
            rules: vec![].into(),
            sub_rules: vec![].into(),
            view,
            frame: view.frame().to_rglica(),
            s_content: s_content.to_rglica(),
            all_margin: Default::default(),
            has: Default::default(),
        }
    }

    pub fn clear(&self) -> &Self {
        self.rules.borrow_mut().clear();
        self.sub_rules.borrow_mut().clear();
        *self.has.borrow_mut() = Default::default();
        self
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
    pub fn back(&self) -> &Self {
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

    pub fn relative_width(&self, view: impl Deref<Target = impl View>, multiplier: impl IntoF32) -> &Self {
        self.relative(Anchor::Width, multiplier, view)
    }

    pub fn relative_size(&self, view: impl Deref<Target = impl View>, multiplier: impl IntoF32) -> &Self {
        self.relative(Anchor::Size, multiplier, view)
    }

    pub fn relative_y(&self, position: impl IntoF32) -> &Self {
        let position = position.into_f32();
        self.custom(move |mut view, s_content| {
            view.frame.origin.y = s_content.height * position;
        })
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

    pub fn center_x(&self) -> &Self {
        self.rules().push(Anchor::CenterX.into());
        self
    }

    pub fn center_y(&self) -> &Self {
        self.rules().push(Anchor::CenterY.into());
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

    pub fn distribute_ratio(&self, ratios: &[impl IntoF32]) -> &Self {
        self.sub_rules()
            .push(Tiling::Distribute(ratios.iter().map(|f| f.into_f32()).collect_vec()).into());
        self
    }

    pub fn all(&self, margin: impl IntoF32) -> &Self {
        *self.all_margin.borrow_mut() = margin.into_f32();
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
    pub fn anchor(
        &self,
        side: Anchor,
        view: impl Deref<Target = impl View + ?Sized>,
        offset: impl IntoF32,
    ) -> &Self {
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

    pub fn custom(&self, action: impl FnMut(Weak<dyn View>, &Size) + 'static) -> &Self {
        self.rules().push(LayoutRule::custom(action));
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

    pub fn tb(&self, offset: impl IntoF32) -> &Self {
        self.t(offset).b(offset)
    }

    pub fn tlb(&self, offset: impl IntoF32) -> &Self {
        self.t(offset).l(offset).b(offset)
    }

    pub fn blt(&self, offset: impl IntoF32) -> &Self {
        self.b(offset).l(offset).t(offset)
    }

    pub fn trb(&self, offset: impl IntoF32) -> &Self {
        self.t(offset).r(offset).b(offset)
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
        self.anchor(Anchor::Bot, view, offset)
    }

    pub fn below(&self, view: impl Deref<Target = impl View> + Copy, offset: impl IntoF32) -> &Self {
        self.anchor(Anchor::Top, view, offset)
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

        for rule in this.rules().iter_mut() {
            if let Some(custom) = &mut rule.custom {
                custom(self.view, &self.s_content)
            } else if rule.between {
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
        let s_content = self.s_content.deref();
        match rule.side {
            Anchor::Top => frame.origin.y = rule.offset,
            Anchor::Bot => {
                if has.height {
                    frame.origin.y = s_content.height - frame.height() - rule.offset
                } else {
                    frame.size.height = frame.height() + s_content.height - frame.max_y() - rule.offset
                }
            }
            Anchor::Left => frame.origin.x = rule.offset,
            Anchor::Right => {
                if has.width {
                    frame.origin.x = s_content.width - frame.width() - rule.offset;
                } else {
                    frame.size.width = frame.width() + s_content.width - frame.max_x() - rule.offset
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
            Tiling::Background => *frame = (*self.s_content.deref()).into(),
            Tiling::Horizontally => place_horizontally(self.view.subviews_mut(), *self.all_margin.borrow()),
            Tiling::Vertically => place_vertically(self.view.subviews_mut(), *self.all_margin.borrow()),
            Tiling::Distribute(ratio) => {
                distribute_with_ratio(self.frame.size, self.view.subviews_mut(), ratio)
            }
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
                self.s_content.height - (self.s_content.height - f.max_y()) / 2.0,
            )),
            Anchor::Left => self.frame.set_center((f.x() / 2.0, cen.y)),
            Anchor::Right => self.frame.set_center((
                self.s_content.width - (self.s_content.width - f.max_x()) / 2.0,
                cen.y,
            )),
            _ => unimplemented!(),
        }
    }
}

fn place_vertically(views: &mut [Own<dyn View>], margin: f32) {
    distribute::<{ Axis::Y }>(views, margin);
}

fn place_horizontally(views: &mut [Own<dyn View>], margin: f32) {
    distribute::<{ Axis::X }>(views, margin);
}

fn distribute<const AXIS: Axis>(views: &mut [Own<dyn View>], margin: f32) {
    let Some(mut last) = views.last_mut().map(|v| v.weak_view()) else {
        return;
    };

    let super_frame = *last.superview().frame();

    if views.len() == 1 {
        let back = super_frame.with_zero_origin();
        last.set_frame(back);
        return;
    }

    let all_margins = margin * (views.len() - 1) as f32;

    let left_length = super_frame.length::<AXIS>() - all_margins;

    let length = left_length / views.len() as f32;
    let other_length = super_frame.other_length::<AXIS>();

    let mut last_pos: f32 = 0.0;

    for frame in views.iter_mut().map(|v| v.frame_mut()) {
        frame.set_position::<AXIS>(last_pos);
        frame.set_other_position::<AXIS>(0);
        frame.set_length::<AXIS>(length);
        frame.set_other_length::<AXIS>(other_length);

        last_pos += length + margin;
    }
}

fn distribute_with_ratio(size: Size, views: &mut [Own<dyn View>], ratios: &[f32]) {
    assert_eq!(
        views.len(),
        ratios.len(),
        "distribute_ratio should have the same number as subviews"
    );

    let total_ratio = 1.0 / ratios.iter().sum::<f32>();

    for i in 0..views.len() {
        let is_first = i == 0;
        let x_pos = if is_first { 0.0 } else { views[i - 1].max_x() };
        views[i].set_frame((x_pos, 0, ratios[i] * size.width * total_ratio, size.height));
    }
}

impl Debug for Placer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.rules.borrow().fmt(f)
    }
}
