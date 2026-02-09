use test_engine::{
    refs::Weak,
    ui::{Container, LIGHT_BLUE, NumberView, Setup, Style, UIManager, ViewData, ViewSubviews, view},
};

use crate::interface::test_game_view::HAS_BACK_BUTTON;

const CORNER_STYLE: Style = Style::new(|v| {
    v.set_color(LIGHT_BLUE).place().size(80, 80);
});

#[view]
pub struct RootLayoutView {
    #[init]
    scale: NumberView,
}

impl Setup for RootLayoutView {
    fn setup(self: Weak<Self>) {
        UIManager::enable_debug_frames();
        UIManager::root_view().set_image("square.png");

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

impl Drop for RootLayoutView {
    fn drop(&mut self) {
        UIManager::disable_debug_frames();
    }
}

pub mod test {

    use anyhow::Result;
    use test_engine::{
        ui::{ViewTest, view_test},
        ui_test::check_colors,
    };

    use super::{RootLayoutView, Setup, ViewData, Weak};

    #[view_test]
    struct RootLayoutViewTest {
        #[init]
        view: RootLayoutView,
    }

    impl Setup for RootLayoutViewTest {
        fn setup(self: Weak<Self>) {
            self.view.place().back();
        }
    }

    impl ViewTest for RootLayoutViewTest {
        fn perform_test(_view: Weak<Self>) -> Result<()> {
            check_colors(
                r"
                          26  273 -   0 218 255
                          27  258 - 121 119 244
                          24   88 - 111 123 231
                          38   53 -   0 218 255
                          95   38 - 119 130 247
                    ",
            )?;

            // test_engine::ui_test::record_ui_test();

            Ok(())
        }
    }
}
