use std::{
    cell::{RefCell, RefMut},
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
};

use gm::{LossyConvert, ToF32, axis::Axis, flat::Size};
use refs::{Rglica, ToRglica, Weak};

use crate::{
    View, ViewSubviews, WeakView,
    layout::{Anchor, Tiling, layout_rule::LayoutRule},
    view::ViewFrame,
};

pub struct Placer {
    pub(crate) rules:            RefCell<Vec<LayoutRule>>,
    pub(crate) all_tiling_rules: RefCell<Vec<LayoutRule>>,

    // Since `Placer` is owned by `View` this should be OK. I hope.
    view:      Rglica<dyn View>,
    s_content: Rglica<Size>,

    all_margin: RefCell<f32>,

    has: RefCell<Size<bool>>,
}

impl Placer {
    pub fn empty() -> Self {
        Self {
            rules:            RefCell::new(vec![]),
            all_tiling_rules: RefCell::new(vec![]),
            view:             Rglica::default(),
            s_content:        Rglica::default(),
            all_margin:       RefCell::new(0.0),
            has:              RefCell::new(Size::default()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rules.borrow().is_empty() && self.all_tiling_rules.borrow().is_empty()
    }

    pub(crate) fn is_ok(&self) -> bool {
        self.view.is_ok()
    }

    pub fn init(&mut self, view: WeakView) {
        let s_content = view.base_view().superview.content_size();
        self.view = unsafe { view.to_rglica() };
        self.s_content = s_content.to_rglica();
    }

    pub fn clear(&self) -> &Self {
        self.rules.borrow_mut().clear();
        self.all_tiling_rules.borrow_mut().clear();
        *self.has.borrow_mut() = Size::default();
        self
    }

    fn rules(&self) -> RefMut<Vec<LayoutRule>> {
        self.rules.borrow_mut()
    }

    fn all_tiling_rules(&self) -> RefMut<Vec<LayoutRule>> {
        self.all_tiling_rules.borrow_mut()
    }

    fn has(&self) -> RefMut<Size<bool>> {
        self.has.borrow_mut()
    }
}

impl Placer {
    pub fn t(&self, offset: impl ToF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::Top, offset.to_f32()));
        self
    }

    pub fn b(&self, offset: impl ToF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::Bot, offset.to_f32()));
        self
    }

    pub fn l(&self, offset: impl ToF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::Left, offset.to_f32()));
        self
    }

    pub fn r(&self, offset: impl ToF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::Right, offset.to_f32()));
        self
    }

    pub fn size(&self, width: impl ToF32, height: impl ToF32) -> &Self {
        self.view.weak_view().set_size(width, height);
        self.w(width).h(height)
    }

    pub fn same_size(&self, view: impl Deref<Target = impl View>) -> &Self {
        self.relative(Anchor::Size, view, 1)
    }

    pub fn same_x(&self, view: impl Deref<Target = impl View>) -> &Self {
        self.anchor(Anchor::X, view, 1)
    }

    pub fn same_y(&self, view: impl Deref<Target = impl View>) -> &Self {
        self.anchor(Anchor::Y, view, 1)
    }

    pub fn relative_width(&self, view: impl Deref<Target = impl View>, multiplier: impl ToF32) -> &Self {
        self.relative(Anchor::Width, view, multiplier)
    }

    pub fn relative_height(&self, view: impl Deref<Target = impl View>, multiplier: impl ToF32) -> &Self {
        self.relative(Anchor::Height, view, multiplier)
    }

    pub fn relative_size(&self, view: impl Deref<Target = impl View>, multiplier: impl ToF32) -> &Self {
        self.relative(Anchor::Size, view, multiplier)
    }

    pub fn same<const S: usize>(
        &self,
        anchors: [Anchor; S],
        view: impl Deref<Target = impl View> + Copy,
    ) -> &Self {
        for anchor in anchors {
            self.relative(anchor, view, 1);
        }
        self
    }

    pub fn w(&self, w: impl ToF32) -> &Self {
        self.rules().insert(0, LayoutRule::make(Anchor::Width, w));
        self.has().width = true;
        self
    }

    pub fn h(&self, h: impl ToF32) -> &Self {
        self.rules().insert(0, LayoutRule::make(Anchor::Height, h));
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

    pub fn back(&self) -> &Self {
        self.rules().push(Tiling::Background.into());
        self
    }

    pub fn left_half(&self) -> &Self {
        self.rules().push(Tiling::LeftHalf.into());
        self
    }

    pub fn right_half(&self) -> &Self {
        self.rules().push(Tiling::RightHalf.into());
        self
    }

    pub fn all_ver(&self) -> &Self {
        self.all_tiling_rules().push(Tiling::Vertically.into());
        self
    }

    pub fn all_hor(&self) -> &Self {
        self.all_tiling_rules().push(Tiling::Horizontally.into());
        self
    }

    pub fn distribute_ratio<const LEN: usize>(&self, ratios: [impl ToF32; LEN]) -> &Self {
        self.all_tiling_rules()
            .push(Tiling::Distribute(ratios.iter().map(|f| f.to_f32()).collect()).into());
        self
    }

    pub fn all(&self, margin: impl ToF32) -> &Self {
        *self.all_margin.borrow_mut() = margin.to_f32();
        self
    }
}

impl Placer {
    pub fn max_width(&self, w: impl ToF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::MaxWidth, w));
        self
    }

    pub fn max_height(&self, h: impl ToF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::MaxHeight, h));
        self
    }

    pub fn min_width(&self, w: impl ToF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::MinWidth, w));
        self
    }

    pub fn min_height(&self, w: impl ToF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::MinHeight, w));
        self
    }
}

impl Placer {
    pub fn anchor(
        &self,
        side: Anchor,
        view: impl Deref<Target = impl View + ?Sized>,
        offset: impl ToF32,
    ) -> &Self {
        self.rules().push(LayoutRule::anchor(side, offset, view.weak_view()));
        self
    }

    pub fn relative(&self, side: Anchor, view: impl Deref<Target = impl View>, ratio: impl ToF32) -> &Self {
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
    pub fn lr(&self, offset: impl ToF32) -> &Self {
        self.l(offset).r(offset)
    }

    pub fn tl(&self, offset: impl ToF32) -> &Self {
        self.t(offset).l(offset)
    }

    pub fn tr(&self, offset: impl ToF32) -> &Self {
        self.t(offset).r(offset)
    }

    pub fn bl(&self, offset: impl ToF32) -> &Self {
        self.b(offset).l(offset)
    }

    pub fn br(&self, offset: impl ToF32) -> &Self {
        self.b(offset).r(offset)
    }

    pub fn tb(&self, offset: impl ToF32) -> &Self {
        self.t(offset).b(offset)
    }

    pub fn tlb(&self, offset: impl ToF32) -> &Self {
        self.t(offset).l(offset).b(offset)
    }

    pub fn blt(&self, offset: impl ToF32) -> &Self {
        self.b(offset).l(offset).t(offset)
    }

    pub fn trb(&self, offset: impl ToF32) -> &Self {
        self.t(offset).r(offset).b(offset)
    }

    pub fn lrt(&self, offset: impl ToF32) -> &Self {
        self.l(offset).r(offset).t(offset)
    }

    pub fn lrb(&self, offset: impl ToF32) -> &Self {
        self.l(offset).r(offset).b(offset)
    }

    pub fn sides(&self, sides: &str, offset: impl ToF32) -> &Self {
        for ch in sides.chars() {
            match ch {
                't' => {
                    self.t(offset);
                }
                'b' => {
                    self.b(offset);
                }
                'l' => {
                    self.l(offset);
                }
                'r' => {
                    self.r(offset);
                }
                _ => panic!("Invalid side. Use letters tblr"),
            }
        }
        self
    }

    pub fn all_sides(&self, offset: impl ToF32) -> &Self {
        self.t(offset).b(offset).l(offset).r(offset)
    }

    pub fn dump_rules(&self) {
        let rules = format!("{:?}", self.rules.borrow());
        let tiling_rules = format!("{:?}", self.all_tiling_rules.borrow());
        println!("Rules: {rules}\nAll tiling rules: {tiling_rules}");
    }
}

impl Placer {
    pub fn above(&self, view: impl Deref<Target = impl View> + Copy, offset: impl ToF32) -> &Self {
        self.same([Anchor::Size, Anchor::X], view).anchor(Anchor::Bot, view, offset)
    }

    pub fn below(&self, view: impl Deref<Target = impl View> + Copy, offset: impl ToF32) -> &Self {
        self.same([Anchor::Size, Anchor::X], view).anchor(Anchor::Top, view, offset)
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
            .push(LayoutRule::between(view.weak_view(), Weak::default(), anchor));
        self
    }
}

impl Placer {
    pub fn layout(&mut self) {
        let this = self.to_rglica();

        let has_left = self.has_left();

        for rule in this.rules().iter_mut() {
            if rule.between {
                self.between_layout(rule);
            } else if rule.anchor_view.is_ok() {
                if rule.relative {
                    self.relative_layout(rule);
                } else {
                    self.anchor_layout(rule, has_left);
                }
            } else if let Some(tiling) = &rule.tiling {
                self.tiling_layout(tiling);
            } else {
                self.simple_layout(rule);
            }
        }

        for rule in this.all_tiling_rules().iter() {
            self.tiling_layout(rule.tiling.as_ref().expect("BUG"));
        }
    }
}

impl Placer {
    fn simple_layout(&mut self, rule: &LayoutRule) {
        let has = *self.has();
        let s_content = self.s_content.deref();

        let view = self.view.deref_mut();
        let mut frame = *view.frame();

        match rule.side {
            Anchor::Top => frame.origin.y = rule.offset,
            Anchor::Bot => {
                if has.height {
                    frame.origin.y = s_content.height - frame.height() - rule.offset;
                } else {
                    frame.size.height = frame.height() + s_content.height - frame.max_y() - rule.offset;
                }
            }
            Anchor::Left => frame.origin.x = rule.offset,
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
            Anchor::Size | Anchor::X | Anchor::Y | Anchor::None => {
                unimplemented!()
            }
        }

        view.set_frame(frame);
    }

    fn anchor_layout(&mut self, rule: &LayoutRule, has_left: bool) {
        let view = self.view.deref_mut();
        let mut frame = *view.frame();
        let a_frame = rule.anchor_view.frame();
        match rule.side {
            Anchor::Top => frame.origin.y = a_frame.max_y() + rule.offset,
            Anchor::Bot => frame.origin.y = a_frame.y() - rule.offset - frame.height(),
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
            _ => unimplemented!(),
        }
        view.set_frame(frame);
    }

    fn relative_layout(&mut self, rule: &LayoutRule) {
        let view = self.view.deref_mut();
        let mut frame = *view.frame();
        let a_frame = rule.anchor_view.frame();
        match rule.side {
            Anchor::Width => frame.size.width = a_frame.size.width * rule.offset,
            Anchor::Height => frame.size.height = a_frame.size.height * rule.offset,
            Anchor::Size => frame.size = a_frame.size * rule.offset,
            Anchor::X => frame.origin.x = a_frame.origin.x * rule.offset,
            Anchor::Y => frame.origin.y = a_frame.origin.y * rule.offset,
            Anchor::CenterY => {
                let s_content = self.s_content.deref();
                let mut center = s_content.center();
                center.y += rule.offset;
                frame.set_center(center);
            }
            _ => unimplemented!(),
        }
        view.set_frame(frame);
    }

    fn tiling_layout(&mut self, tiling: &Tiling) {
        let mut frame = *self.view.frame();
        match tiling {
            Tiling::Background => frame = (*self.s_content.deref()).into(),
            Tiling::Horizontally => place_horizontally(self.view.subviews_mut(), *self.all_margin.borrow()),
            Tiling::Vertically => place_vertically(self.view.subviews_mut(), *self.all_margin.borrow()),
            Tiling::LeftHalf => frame = (0, 0, self.s_content.width / 2.0, self.s_content.height).into(),
            Tiling::RightHalf => {
                frame = (
                    self.s_content.width / 2.0,
                    0,
                    self.s_content.width / 2.0,
                    self.s_content.height,
                )
                    .into();
            }
            Tiling::Distribute(ratio) => distribute_with_ratio(frame.size, self.view.subviews_mut(), ratio),
        }
        self.view.set_frame(frame);
    }

    fn between_layout(&mut self, rule: &LayoutRule) {
        if rule.side.is_none() {
            self.between_2_layout(rule);
        } else {
            self.between_s_layout(rule);
        }
    }

    fn between_2_layout(&mut self, rule: &LayoutRule) {
        let center_a = rule.anchor_view.frame().center();
        let center_b = rule.anchor_view2.frame().center();
        let center = center_a.middle(&center_b);
        self.view.edit_frame(|frame| frame.set_center(center));
    }

    fn between_s_layout(&mut self, rule: &LayoutRule) {
        let f = rule.anchor_view.frame();
        let cen = f.center();

        let view = self.view.deref_mut();
        let mut frame = *view.frame();

        match rule.side {
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
            _ => unimplemented!(),
        }
        view.set_frame(frame);
    }
}

impl Placer {
    fn has_left(&self) -> bool {
        self.rules.borrow().iter().any(|rule| rule.side.is_left())
    }
}

fn place_vertically(views: Vec<WeakView>, margin: f32) {
    distribute::<{ Axis::Y }>(views, margin);
}

fn place_horizontally(views: Vec<WeakView>, margin: f32) {
    distribute::<{ Axis::X }>(views, margin);
}

fn distribute<const AXIS: Axis>(mut views: Vec<WeakView>, margin: f32) {
    let Some(mut last) = views.last_mut().map(|v| v.weak_view()) else {
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

fn distribute_with_ratio(size: Size, mut views: Vec<WeakView>, ratios: &[f32]) {
    let total_ratio = 1.0 / ratios.iter().sum::<f32>();

    for i in 0..ratios.len() {
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
