use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{BLACK, BLUE, Button, Container, GREEN, HasText, RED, Setup, UI, ViewData, ViewSubviews, view},
    ui_test::check_colors,
};

#[view]
struct TilingLayout {
    #[init]
    menu: Container,
}

impl Setup for TilingLayout {
    fn setup(mut self: Weak<Self>) {
        self.menu.set_color(BLACK).place().tl(20).size(280, 280).all_ver();

        self.menu.add_view::<Container>().set_color(RED);
        self.menu.add_view::<Container>().set_color(GREEN);
        self.menu.add_view::<Container>().set_color(BLUE);
    }
}

pub async fn test_tiling_layout() -> anyhow::Result<()> {
    let mut view = UI::init_test_view::<TilingLayout>().await;

    check_colors(
        r#"
               6   65 -  89 124 149
              13   65 -  89 124 149
              60   67 - 255   0   0
             127   68 - 255   0   0
             219   68 - 255   0   0
             304   74 -  89 124 149
             322   96 -  89 124 149
             319  155 -  89 124 149
             266  170 -   0 255   0
             150  173 -   0 255   0
              75  171 -   0 255   0
              48  171 -   0 255   0
              34  170 -   0 255   0
              12  170 -  89 124 149
               8  174 -  89 124 149
              10  215 -  89 124 149
              15  253 -  89 124 149
              77  263 -   0   0 231
             230  256 -   0   0 231
             270  251 -   0   0 231
             332  252 -  89 124 149
             336  273 -  89 124 149
             217  347 -  89 124 149
             192  326 -  89 124 149
             174  317 -  89 124 149
             153  289 -   0   0 231
             152  273 -   0   0 231
             150  262 -   0   0 231
             149  239 -   0   0 231
             146  218 -   0   0 231
             143  160 -   0 255   0
             143  158 -   0 255   0
             142  115 -   0 255   0
             140   71 - 255   0   0
             139   63 - 255   0   0
             139   52 - 255   0   0
             138   33 - 255   0   0
             136   16 -  89 124 149
             136    7 -  89 124 149
        "#,
    )
    .await?;

    from_main(move || {
        view.menu.remove_all_subviews();
    })
    .await;

    check_colors(
        r#"
             164  331 -  89 124 149
             163  325 -  89 124 149
             160  287 -   0   0   0
             161  228 -   0   0   0
             191  193 -   0   0   0
             253  146 -   0   0   0
             298  114 -   0   0   0
             336   82 -  89 124 149
             333   66 -  89 124 149
             307   37 -  89 124 149
             270   19 -  89 124 149
             251   15 -  89 124 149
             231   17 -  89 124 149
             192   21 -   0   0   0
             123   22 -   0   0   0
              76   56 -   0   0   0
              45  110 -   0   0   0
              24  118 -   0   0   0
               9  129 -  89 124 149
               7  138 -  89 124 149
              18  177 -  89 124 149
              71  200 -   0   0   0
             172  124 -   0   0   0
             240   92 -   0   0   0
             213  193 -   0   0   0
        "#,
    )
    .await?;

    from_main(move || {
        view.menu
            .add_view::<Button>()
            .add_transition::<TilingLayout, TilingLayout>()
            .set_text("Classic")
            .set_text_size(80);

        view.menu
            .add_view::<Button>()
            .add_transition::<TilingLayout, TilingLayout>()
            .set_text("Custom Game")
            .set_text_size(80);

        view.menu
            .add_view::<Button>()
            .add_transition::<TilingLayout, TilingLayout>()
            .set_text("Settings")
            .set_text_size(80);
    })
    .await;

    check_colors(
        r#"
             303  250 -  89 124 149
             292  252 - 130 130 130
             257  258 -  14  14  14
             226  258 -  14  14  14
             198  259 - 255 255 255
             164  260 -   0   0   0
             105  256 - 223 223 223
              98  257 - 255 255 255
              62  253 -   0   0   0
              52  252 - 210 210 210
              33  252 - 255 255 255
              34  247 - 185 185 185
              41  212 - 255 255 255
              38  186 - 255 255 255
              43  147 - 255 255 255
              64  140 - 255 255 255
              94  145 - 255 255 255
             134  148 -   0   0   0
             163  153 - 255 255 255
             212  158 - 255 255 255
             226  160 -  14  14  14
             243  160 - 210 210 210
             268  158 - 255 255 255
             284  159 - 255 255 255
             298  148 - 255 255 255
             307   98 -  89 124 149
             302   69 -  89 124 149
             278   51 -  14  14  14
             233   50 - 255 255 255
             209   51 - 204 204 204
             184   56 -   0   0   0
             151   58 - 255 255 255
             138   61 -   0   0   0
             119   62 - 255 255 255
             103   68 - 255 255 255
              87   74 - 255 255 255
              78   78 -  14  14  14
              65   85 -  14  14  14
              63   85 -  14  14  14
              63   81 - 255 255 255
        "#,
    )
    .await?;

    Ok(())
}
