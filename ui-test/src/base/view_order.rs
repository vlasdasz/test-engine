use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, Color, Container, UIManager, ViewData, ViewFrame, ViewSetup, ViewSubviews, UI},
    ui_test::helpers::check_colors,
};

#[view]
pub struct OrderTestView {
    #[init]
    view_1: Container,
    view_2: Container,
    view_3: Container,
    view_4: Container,
}

impl ViewSetup for OrderTestView {
    fn setup(mut self: Weak<Self>) {
        self.view_1.set_color(Color::RED).place().size(200, 200);
        self.view_2.set_color(Color::GREEN).place().size(200, 200).tl(100);
        self.view_3.set_color(Color::BLUE).place().size(200, 200).tl(200);
        self.view_4.set_color(Color::BLACK).place().size(200, 200).tl(300);
    }
}

pub async fn test_view_order() -> Result<()> {
    let view = UI::init_test_view::<OrderTestView>().await;

    assert_eq!(
        view.dump_subviews(),
        vec![
            "OrderTestView.view_1: Container".to_string(),
            "OrderTestView.view_2: Container".to_string(),
            "OrderTestView.view_3: Container".to_string(),
            "OrderTestView.view_4: Container".to_string()
        ]
    );

    assert_eq!(
        view.z_position(),
        UIManager::root_view().z_position() - UIManager::subview_z_offset()
    );

    assert_eq!(
        view.view_1.z_position(),
        view.z_position() - UIManager::subview_z_offset()
    );

    assert_eq!(view.view_1.z_position(), view.view_2.z_position());
    assert_eq!(view.view_2.z_position(), view.view_3.z_position());
    assert_eq!(view.view_3.z_position(), view.view_4.z_position());

    assert_eq!(view.view_1.view_label, "OrderTestView.view_1: Container");
    assert_eq!(view.view_2.view_label, "OrderTestView.view_2: Container");
    assert_eq!(view.view_3.view_label, "OrderTestView.view_3: Container");
    assert_eq!(view.view_4.view_label, "OrderTestView.view_4: Container");

    assert_eq!(view.subviews()[0].label(), view.view_1.view_label);
    assert_eq!(view.subviews()[1].label(), view.view_2.view_label);
    assert_eq!(view.subviews()[2].label(), view.view_3.view_label);
    assert_eq!(view.subviews()[3].label(), view.view_4.view_label);

    check_colors(
        r"
              34   25 - 255   0   0
              35   26 - 255   0   0
              38   59 - 255   0   0
              53   59 - 255   0   0
              92   90 - 255   0   0
             102  110 -   0 255   0
             120  114 -   0 255   0
             133  128 -   0 255   0
             160  148 -   0 255   0
             190  180 -   0 255   0
             205  193 -   0 255   0
             215  231 -   0   0 203
             301  290 -   0   0 203
             309  328 -   0   0   0
             340  358 -   0   0   0
             401  416 -   0   0   0
             456  480 -   0   0   0
             485  498 -   0   0   0
             502  523 -  25  51  76
             518  523 -  25  51  76
    ",
    )
    .await?;

    debug!("View order: OK");

    Ok(())
}
