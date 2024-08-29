use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, Anchor::Y, Color, Container, ViewData, ViewSetup, ViewSubviews, UI},
    ui_test::record_ui_test,
};

#[view]
struct CenterFieldTestView {

    field: Weak<Container>,

    #[init]
    container: Container,
}

impl ViewSetup for CenterFieldTestView {
    fn setup(mut self: Weak<Self>) {

        self.container.set_color(Color::GREEN);
        self.container.place().all_sides(100);


        self.field = self.container.add_view();

        self.field.set_color(Color::BLUE);
        self.field.place().lr(20).h(68).max_width(400).center_x().relative(Y, self.container, 2.0);
    }
}

pub async fn test_center_field() -> anyhow::Result<()> {
    let _view = UI::init_test_view::<CenterFieldTestView>().await;

    record_ui_test().await;

    debug!("Center field: OK");

    Ok(())
}
