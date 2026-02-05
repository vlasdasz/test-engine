use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{BLUE, Container, GREEN, Setup, UIDrawer, ViewData, ViewFrame, ViewSubviews, view},
    ui_test::check_colors,
};

#[view]
struct RelativeLayout {
    view: Weak<Container>,

    #[init]
    parent: Container,
}

impl Setup for RelativeLayout {
    fn setup(mut self: Weak<Self>) {
        self.parent.set_color(BLUE);
        self.parent.set_frame((50, 50, 200, 200));

        self.view = self.parent.add_view();

        self.view.set_color(GREEN);
        self.view
            .place()
            .relative_size(self.parent, 0.4)
            .relative_x(0.2)
            .relative_y(0.5);
    }
}

pub async fn test_relative_layout() -> Result<()> {
    let view = UIDrawer::init_test_view::<RelativeLayout>();

    check_colors(
        r#"
              41  213 -  89 124 149
              70  209 -   0   0 231
              83  208 -   0   0 231
             104  205 -   0 255   0
             148  205 -   0 255   0
             179  205 -   0   0 231
             222  205 -   0   0 231
             271  206 -  89 124 149
             130  276 -  89 124 149
             130  255 -  89 124 149
             130  239 -   0   0 231
             130  211 -   0 255   0
             130  177 -   0 255   0
             130  121 -   0   0 231
             130   87 -   0   0 231
        "#,
    )?;

    from_main(move || {
        view.parent.set_size(280, 400);
    });

    check_colors(
        r#"
             158  462 -  89 124 149
             158  429 -   0   0 231
             158  373 -   0 255   0
             164  284 -   0 255   0
             164  230 -   0   0 231
              35  339 -  89 124 149
              72  339 -   0   0 231
             133  339 -   0 255   0
             240  337 -   0   0 231
             361  338 -  89 124 149
             241  212 -   0   0 231
              83  211 -   0   0 231
              80  430 -   0   0 231
              68  472 -  89 124 149
              24  466 -  89 124 149
             175  302 -   0 255   0
              94   39 -  89 124 149
              34  151 -  89 124 149
        "#,
    )?;

    Ok(())
}
