use log::debug;
use test_engine::{
    from_main,
    refs::Weak,
    ui::{view, Anchor::CenterY, Color, Container, Setup, ViewData, ViewSubviews, UI},
    ui_test::check_colors,
};

#[view]
struct CenterFieldTestView {
    field: Weak<Container>,

    #[init]
    container: Container,
}

impl Setup for CenterFieldTestView {
    fn setup(mut self: Weak<Self>) {
        self.container.set_color(Color::GREEN);
        self.container.place().all_sides(100);

        self.field = self.container.add_view();

        self.field.set_color(Color::BLUE);
        self.field.place().lr(20).h(68);
    }
}

pub async fn test_center_field() -> anyhow::Result<()> {
    let view = UI::init_test_view::<CenterFieldTestView>().await;

    check_colors(
        r#"
              89  155 -  25  51  76
             111  152 -   0 255   0
             128  149 -   0   0 203
             137   94 -  25  51  76
             130  119 -   0   0 203
             108  118 -   0 255   0
              81  118 -  25  51  76
             134  191 -   0 255   0
             135  148 -   0   0 203
             116  174 -   0 255   0
             412   81 -  25  51  76
             438  130 -   0   0 203
             485  114 -   0 255   0
             516  113 -  25  51  76
             516  153 -  25  51  76
             491  153 -   0 255   0
             468  152 -   0   0 203
             438  194 -   0 255   0
             286  194 -   0 255   0
             286  135 -   0   0 203
             201  135 -   0   0 203
             395  135 -   0   0 203
             304   55 -  25  51  76
        "#,
    )
    .await?;

    from_main(move || {
        view.container.place().clear().all_sides(200);
    })
    .await;

    check_colors(
        r#"
             179  238 -  25  51  76
             210  233 -   0 255   0
             233  229 -   0   0 203
             303  229 -   0   0 203
             366  229 -   0   0 203
             391  227 -   0 255   0
             424  226 -  25  51  76
             329  293 -   0 255   0
             329  250 -   0   0 203
             324  208 -   0   0 203
             323  170 -  25  51  76
             151  224 -  25  51  76
             212  228 -   0 255   0
             326  238 -   0   0 203
             446  237 -  25  51  76
             424  293 -  25  51  76
             312  293 -   0 255   0
             273  243 -   0   0 203
             208  209 -   0 255   0
             219  175 -  25  51  76
             370  179 -  25  51  76
             162  260 -  25  51  76
             286  303 -   0 255   0
        "#,
    )
    .await?;

    from_main(move || {
        view.container.place().clear().all_sides(250);
        view.field.place().max_width(200);
    })
    .await;

    check_colors(
        r#"
             229  271 -  25  51  76
             263  277 -   0 255   0
             297  275 -   0   0 203
             341  272 -   0 255   0
             374  272 -  25  51  76
             299  364 -  25  51  76
             299  325 -   0 255   0
             299  299 -   0   0 203
             302  260 -   0   0 203
             300  219 -  25  51  76
             256  232 -  25  51  76
             229  286 -  25  51  76
        "#,
    )
    .await?;

    from_main(move || {
        view.container.place().clear().all_sides(100);
    })
    .await;

    check_colors(
        r#"
              69  135 -  25  51  76
             111  126 -   0 255   0
             132  125 -   0   0 203
             309  126 -   0   0 203
             327  126 -   0 255   0
             409  127 -   0 255   0
             486  128 -   0 255   0
             527  128 -  25  51  76
             203  537 -  25  51  76
             202  421 -   0 255   0
             200  305 -   0 255   0
             194  180 -   0 255   0
             191  161 -   0   0 203
             195  113 -   0   0 203
             193   61 -  25  51  76
        "#,
    )
    .await?;

    from_main(move || {
        view.field.place().center_x();
    })
    .await;

    check_colors(
        r#"
              82  129 -  25  51  76
             113  126 -   0 255   0
             140  124 -   0 255   0
             191  124 -   0 255   0
             212  122 -   0   0 203
             286  123 -   0   0 203
             391  131 -   0   0 203
             410  130 -   0 255   0
             481  124 -   0 255   0
             519  124 -  25  51  76
             315  198 -   0 255   0
             329  132 -   0   0 203
             358   69 -  25  51  76
             388   69 -  25  51  76
             384  123 -   0   0 203
             384  178 -   0 255   0
             214  183 -   0 255   0
             210  132 -   0   0 203
             216   69 -  25  51  76
        "#,
    )
    .await?;

    from_main(move || {
        view.field.place().relative(CenterY, view.container, -50.0);
    })
    .await;

    check_colors(
        r#"
              86  259 -  25  51  76
             124  253 -   0 255   0
             189  245 -   0 255   0
             216  245 -   0   0 203
             325  241 -   0   0 203
             383  256 -   0   0 203
             421  250 -   0 255   0
             382  295 -   0 255   0
             380  248 -   0   0 203
             377  189 -   0 255   0
             299  197 -   0 255   0
             286  227 -   0   0 203
             281  274 -   0   0 203
             276  295 -   0 255   0
             219  289 -   0 255   0
             216  270 -   0   0 203
             220  235 -   0   0 203
             221  205 -   0 255   0
             185  207 -   0 255   0
             179  262 -   0 255   0
             238  301 -   0 255   0
        "#,
    )
    .await?;

    debug!("Center field: OK");

    Ok(())
}
