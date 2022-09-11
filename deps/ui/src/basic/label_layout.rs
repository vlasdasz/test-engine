use std::ops::{Deref, DerefMut};

use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use gm::flat::Size;
use smart_default::SmartDefault;
use text::Font;

#[derive(SmartDefault)]
pub(crate) struct LabelLayout {
    #[default = 64.0]
    size:   f32,
    #[default(Layout::new(CoordinateSystem::PositiveYDown))]
    layout: Layout,
}

impl LabelLayout {
    pub(crate) fn clear(&mut self) {
        self.reset(&LayoutSettings {
            ..LayoutSettings::default()
        });
    }

    pub(crate) fn set_text(&mut self, font: &Font, text: &str) {
        self.clear();
        self.layout
            .append(&[&font.font], &TextStyle::new(text, self.size, 0));
    }

    pub(crate) fn size(&mut self) -> Size {
        let last = self.glyphs().last().unwrap();
        (last.x + last.width as f32, last.y + last.height as f32).into()
    }
}

impl Deref for LabelLayout {
    type Target = Layout;
    fn deref(&self) -> &Layout {
        &self.layout
    }
}

impl DerefMut for LabelLayout {
    fn deref_mut(&mut self) -> &mut Layout {
        &mut self.layout
    }
}
