use anyhow::Result;
use test_engine::{
    AppRunner,
    dispatch::from_main,
    inspect::views::PlacerView,
    refs::Weak,
    ui::{Container, Setup, TURQUOISE, UI, ViewData, ViewFrame, view},
    ui_test::record_ui_test,
};

#[view]
struct PlacerViewTest {
    #[init]
    placer_view: PlacerView,
    view:        Container,
}

impl Setup for PlacerViewTest {
    fn setup(mut self: Weak<Self>) {
        self.placer_view.set_size(200, 800);

        self.view.set_color(TURQUOISE);
        self.view.place().l(250).t(100).size(80, 200);
    }
}

pub(crate) async fn test_placer_view() -> Result<()> {
    let view = UI::init_test_view::<PlacerViewTest>();
    // UIManager::enable_debug_frames();
    AppRunner::set_window_size((1000, 1000));

    from_main(move || {
        view.placer_view.set_placer("sokol", &view.view.place());
    });

    record_ui_test();

    Ok(())
}
