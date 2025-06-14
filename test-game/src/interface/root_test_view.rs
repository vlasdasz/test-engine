use test_engine::{
    refs::Weak,
    ui::{Container, LIGHT_BLUE, NumberView, Setup, Style, UIManager, ViewData, ViewSubviews, view},
};

use crate::interface::test_game_view::HAS_BACK_BUTTON;

const CORNER_STYLE: Style = Style::new(|v| {
    v.set_color(LIGHT_BLUE).place().size(80, 80);
});

#[view]
pub struct RootTestView {
    #[init]
    scale: NumberView,
}

impl Setup for RootTestView {
    fn setup(mut self: Weak<Self>) {
        self.apply_style(HAS_BACK_BUTTON);

        self.add_view::<Container>().apply_style(CORNER_STYLE).place().tl(0);
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().tr(0);
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().br(0);
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().bl(0);

        self.add_view::<Container>().apply_style(CORNER_STYLE).place().t(0).center_x();
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().l(0).center_y();
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().r(0).center_y();
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().b(0).center_x();

        self.scale
            .set_min(0.2)
            .set_step(0.1)
            .set_value(1)
            .place()
            .center()
            .size(100, 200);
        self.scale.on_change(|scale| {
            UIManager::set_scale(scale);
        });
    }
}
