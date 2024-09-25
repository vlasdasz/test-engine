use log::debug;
use test_engine::{
    from_main,
    refs::Weak,
    ui::{view, Color, Container, Setup, ViewData, ViewSubviews, UI},
    ui_test::check_colors,
};

#[view]
struct TilingLayoutTestView {
    #[init]
    menu: Container,
}

impl Setup for TilingLayoutTestView {
    fn setup(mut self: Weak<Self>) {
        self.menu.set_color(Color::BLACK).place().all_sides(100).all_ver();

        self.menu.add_view::<Container>().set_color(Color::RED);
        self.menu.add_view::<Container>().set_color(Color::GREEN);
        self.menu.add_view::<Container>().set_color(Color::BLUE);
    }
}

pub async fn test_tiling_layout() -> anyhow::Result<()> {
    let mut view = UI::init_test_view::<TilingLayoutTestView>().await;

    check_colors(
        r#"
              80  469 -  25  51  76
             143  463 -   0   0 203
             263  447 -   0   0 203
             365  450 -   0   0 203
             429  450 -   0   0 203
             521  447 -  25  51  76
             461  501 -  25  51  76
             450  523 -  25  51  76
             450  523 -  25  51  76
             355  518 -  25  51  76
             322  474 -   0   0 203
             237  534 -  25  51  76
             175  464 -   0   0 203
             153  510 -  25  51  76
              85  318 -  25  51  76
             147  322 -   0 255   0
             253  377 -   0   0 203
             420  314 -   0 255   0
             523  310 -  25  51  76
             436  231 - 255   0   0
             479  258 -   0 255   0
             479  195 - 255   0   0
             507  179 -  25  51  76
             507   98 -  25  51  76
             379   72 -  25  51  76
             198   71 -  25  51  76
              78   73 -  25  51  76
              78  177 -  25  51  76
             128  178 - 255   0   0
             127  285 -   0 255   0
              67  266 -  25  51  76
              84  235 -  25  51  76
              84  234 -  25  51  76
             159  196 - 255   0   0
             382  124 - 255   0   0
             472   56 -  25  51  76
             533  172 -  25  51  76
             570  520 -  25  51  76
             361  520 -  25  51  76
        "#,
    )
    .await?;

    from_main(move || {
        view.menu.remove_all_subviews();
    })
    .await;

    check_colors(
        r#"
             199  534 -  25  51  76
             186  363 -   0   0   0
              54  281 -  25  51  76
             185   73 -  25  51  76
             322  219 -   0   0   0
             547  271 -  25  51  76
             393  462 -   0   0   0
             415  535 -  25  51  76
        "#,
    )
    .await?;

    from_main(move || {
        view.menu
            .add_transition::<TilingLayoutTestView, TilingLayoutTestView>()
            .set_text("Classic")
            .set_text_size(80);

        view.menu
            .add_transition::<TilingLayoutTestView, TilingLayoutTestView>()
            .set_text("Custom Game")
            .set_text_size(80);

        view.menu
            .add_transition::<TilingLayoutTestView, TilingLayoutTestView>()
            .set_text("Settings")
            .set_text_size(80);
    })
    .await;

    check_colors(
        r#"
             273  523 -  25  51  76
             273  479 - 255 255 255
             265  432 -   0   0   0
             243  376 - 255 255 255
             235  316 -   1   1   1
             179  272 -   1   1   1
             198  162 - 255 255 255
             198  137 - 255 255 255
             207   80 -  25  51  76
             289  157 - 255 255 255
             325  162 - 255 255 255
             344  162 -   0   0   0
             372  172 - 255 255 255
             382  172 -   0   0   0
             401  171 - 255 255 255
             428  279 - 255 255 255
             423  284 -   1   1   1
             410  307 - 255 255 255
             403  425 - 255 255 255
             420  425 - 255 255 255
             348  432 -   1   1   1
             242  432 -   1   1   1
             179  351 - 255 255 255
             179  297 - 255 255 255
             262  296 - 255 255 255
             363  293 -   1   1   1
             405  293 -   1   1   1
             392  175 -  36  36  36
             160  168 - 255 255 255
              50  181 -  25  51  76
        "#,
    )
    .await?;

    debug!("Tiling layout: OK");

    Ok(())
}
