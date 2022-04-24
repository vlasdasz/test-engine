use rtools::Rglica;

use crate::{placer::Anchor, view::ViewTemplates, view_base::ViewBase, Font, ImageView, View};

#[derive(Default, Debug)]
pub struct LayoutView {
    base:       ViewBase,
    central:    Rglica<ViewBase>,
    satellites: Vec<Rglica<ImageView>>,
}

impl View for LayoutView {
    fn setup(&mut self) {
        let font = Font::default();

        for i in 1..=12 {
            let str: String = format!("{:X}", i);
            let ch = str.chars().last().unwrap();
            let mut view = self.add_view::<ImageView>();
            let image = font.glyph_for_char(ch).image;
            view.set_image(image);
            view.frame_mut().size = (10, 10).into();
            self.satellites.push(view);
        }

        self.central = self.add_view();
        self.central.frame.size = (40, 40).into();
    }

    fn layout(&mut self) {
        let mut c = self.central;
        let s = &mut self.satellites;

        c.place().center();
        c.place().proportional_width(0.6);
        c.place().proportional_height(0.4);

        s[0].place().anchor(c, Anchor::Left, Anchor::Bot, 5);
        s[1].place().anchor(c, Anchor::Left, Anchor::Center, 5);
        s[2].place().anchor(c, Anchor::Left, Anchor::Top, 5);

        s[3].place().anchor(c, Anchor::Top, Anchor::Left, 5);
        s[4].place().anchor(c, Anchor::Top, Anchor::Center, 5);
        s[5].place().anchor(c, Anchor::Top, Anchor::Right, 5);

        s[6].place().anchor(c, Anchor::Right, Anchor::Top, 5);
        s[7].place().anchor(c, Anchor::Right, Anchor::Center, 5);
        s[8].place().anchor(c, Anchor::Right, Anchor::Bot, 5);

        s[9].place().anchor(c, Anchor::Bot, Anchor::Right, 5);
        s[10].place().anchor(c, Anchor::Bot, Anchor::Center, 5);
        s[11].place().anchor(c, Anchor::Bot, Anchor::Left, 5);
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
