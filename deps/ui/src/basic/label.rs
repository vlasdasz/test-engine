use derivative::Derivative;
use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use rtools::{file::File, Rglica};

use crate::{
    view::{ViewData, ViewFrame, ViewSubviews},
    Font, ImageView, View, ViewBase,
};

#[derive(Default, Derivative)]
#[derivative(Debug)]
pub struct Label {
    #[derivative(Debug = "ignore")]
    font:    Font,
    text:    String,
    base:    ViewBase,
    content: Rglica<ViewBase>,
}

impl Label {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl ToString) {
        self.text = text.to_string();
        self.set_shmetters();
    }

    pub fn clear(&mut self) {
        self.set_text("")
    }

    fn set_shmetters(&mut self) {
        self.content.remove_all_subviews();

        let data = File::read(&self.font.path);

        let font = fontdue::Font::from_bytes(data, fontdue::FontSettings::default()).unwrap();

        let mut layout: Layout = Layout::new(CoordinateSystem::PositiveYDown);

        let fonts = &[font];

        layout.reset(&LayoutSettings {
            ..LayoutSettings::default()
        });

        layout.append(fonts, &TextStyle::new(&self.text, 28.0, 0));

        for glyph in layout.glyphs() {
            let mut view = self.content.add_view::<ImageView>();
            view.set_frame((glyph.x, glyph.y, glyph.width, glyph.height));
            view.set_image(self.font.glyph_for_char(glyph.parent).image);
        }
    }
}

impl View for Label {
    fn setup(&mut self) {
        self.content = self.add_view();
        self.set_shmetters();
    }

    fn layout(&mut self) {
        self.content.place().as_background()
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
