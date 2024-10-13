use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, Color, Container, HasTitle, MovableView, Setup, ViewData, ViewFrame, UI},
    ui_test::{check_colors, inject_touches},
};

#[view]
struct MovableViewTestView {
    #[init]
    movable: MovableView<Container>,
}

impl Setup for MovableViewTestView {
    fn setup(mut self: Weak<Self>) {
        self.movable.set_title("Movable view");
        self.movable.set_frame((10, 10, 400, 400));
        self.movable.target_view.set_color(Color::GREEN);
    }
}

pub async fn test_movable_view() -> Result<()> {
    let mut _view = UI::init_test_view::<MovableViewTestView>().await;

    inject_touches(
        "
            346  36   b
            438  90   m
            438  90   e
        ",
    )
    .await;

    check_colors(
        r#"
             521  433 -  25  51  76
             499  436 -   0 255   0
             481  436 -   0 255   0
             474  442 -   0 255   0
             454  448 -   0 255   0
             446  486 -  25  51  76
             143  493 -  25  51  76
             143  449 -   0 255   0
             128  430 -   0 255   0
             111  421 -   0 255   0
              73  419 -  25  51  76
              61  133 -  25  51  76
             133  133 -   0 255   0
             136  115 -   0 255   0
             138   99 - 255 255 255
             150   73 - 255 255 255
             150   34 -  25  51  76
             468   38 -  25  51  76
             465   85 - 255 255 255
             464  117 -   0 255   0
             498  123 -   0 255   0
             537  124 -  25  51  76
        "#,
    )
    .await?;

    inject_touches(
        "
            501  458  b
            323  192  m
            323  192  e
        ",
    )
    .await;

    check_colors(
        r#"
             288  227 -  25  51  76
             288  224 -  25  51  76
             287  145 -   0 255   0
             310  145 -   0 255   0
             331  144 -  25  51  76
             341   87 -  25  51  76
             312   86 - 255 255 255
             311   52 -  25  51  76
              84   90 -  25  51  76
             109   87 - 255 255 255
             112   42 -  25  51  76
              86  170 -  25  51  76
             142  165 -   0 255   0
             182  224 -  25  51  76
             242  219 -  25  51  76
             237  154 -   0 255   0
        "#,
    )
    .await?;

    inject_touches(
        "
            313  190  b
            78   78   m
            78   78   e
        ",
    )
    .await;

    check_colors(
        r#"
             115  176 -  25  51  76
             115  148 -   0 255   0
              92  148 -  25  51  76
             188  173 -  25  51  76
             185  134 -   0 255   0
             217  117 -  25  51  76
             176   49 -  25  51  76
             182   67 - 255 255 255
             226   79 -  25  51  76
              90   92 -  25  51  76
             114   74 -   0   0   0
             115   48 -  25  51  76
        "#,
    )
    .await?;

    debug!("Test movable view: OK");

    Ok(())
}
