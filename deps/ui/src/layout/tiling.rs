use std::ops::DerefMut;

use gm::flat::Rect;
use rtools::ToRglica;

use crate::{layout::TilingRule, View, ViewFrame, ViewSubviews};

#[derive(Default)]
pub struct Tiling {
    pub(crate) rules: Vec<TilingRule>,
}

impl Tiling {
    pub fn hor(&mut self) {
        self.rules.push(TilingRule::Horizontally)
    }

    pub fn ver(&mut self) {
        self.rules.push(TilingRule::Vertically)
    }
}

impl Tiling {
    pub fn layout(&self, frame: &mut Rect, s_frame: &Rect, subviews: &mut [Box<dyn View>]) {
        for rule in &self.rules {
            match rule {
                TilingRule::Background => *frame = s_frame.with_zero_origin(),
                TilingRule::Horizontally => {
                    todo!()
                }
                TilingRule::Vertically => place_vertically(&mut *subviews),
            };
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
