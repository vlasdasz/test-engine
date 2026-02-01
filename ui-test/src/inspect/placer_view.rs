use anyhow::Result;
use test_engine::{
    AppRunner,
    dispatch::from_main,
    inspect::views::PlacerView,
    refs::Weak,
    ui::{Container, Setup, TURQUOISE, UIDrawer, ViewData, ViewFrame, view},
};

#[view]
struct PlacerViewTest {
    #[init]
    placer_view: PlacerView,
    view:        Container,
}

impl Setup for PlacerViewTest {
    fn setup(mut self: Weak<Self>) {
        test_engine::ui::UIManager::override_scale(2.0);

        self.placer_view.set_size(200, 800);

        self.view.set_color(TURQUOISE);
        self.view.place().center().size(80, 200);
    }
}

pub(crate) async fn test_placer_view() -> Result<()> {
    let view = UIDrawer::init_test_view::<PlacerViewTest>();
    // UIManager::enable_debug_frames();
    AppRunner::set_window_size((1000, 1000));

    from_main(move || {
        view.placer_view.set_placer("sokol", &view.view.place());
    });

    test_engine::ui_test::record_ui_test();

    from_main(|| {
        test_engine::ui::UIManager::override_scale(1.0);
    });

    Ok(())
}
