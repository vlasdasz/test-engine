use test_engine::{
    from_main,
    refs::Weak,
    ui::{Color, Container, Setup, UI, ViewData, ViewFrame, ViewSubviews, view},
    ui_test::check_colors,
};

#[view]
struct MinWidth {
    view: Weak<Container>,

    #[init]
    container: Container,
}

impl Setup for MinWidth {
    fn setup(mut self: Weak<Self>) {
        self.container.set_color(Color::GREEN);
        self.container.set_size((400, 400)).set_position((20, 20));

        self.view = self.container.add_view();
        self.view.set_color(Color::BLUE);
        self.view.place().all_sides(150);
    }
}

pub async fn test_min_width() -> anyhow::Result<()> {
    let view = UI::init_test_view::<MinWidth>().await;

    check_colors(
        r#"
             449  227 -  89 124 149
             423  230 -  89 124 149
             339  227 -   0 255   0
             302  228 -   0 255   0
             278  227 -   0 255   0
             246  227 -   0   0 231
             241  227 -   0   0 231
             207  222 -   0   0 231
             181  225 -   0   0 231
             180  225 -   0   0 231
             131  221 -   0 255   0
              86  219 -   0 255   0
              54  220 -   0 255   0
              45  220 -   0 255   0
              21  223 -   0 255   0
               5  224 -  89 124 149
               5  224 -  89 124 149
               5  224 -  89 124 149
        "#,
    )
    .await?;

    from_main(move || {
        view.view.place().min_width(250).center_x();
    })
    .await;

    check_colors(
        r#"
             440  216 -  89 124 149
             433  216 -  89 124 149
             385  212 -   0 255   0
             349  209 -   0 255   0
             340  209 -   0   0 231
             314  209 -   0   0 231
             269  209 -   0   0 231
             224  209 -   0   0 231
             161  210 -   0   0 231
             138  211 -   0   0 231
             103  210 -   0   0 231
              59  209 -   0 255   0
              18  209 -  89 124 149
               7  212 -  89 124 149
             261  501 -  89 124 149
             237  439 -  89 124 149
             228  420 -  89 124 149
             218  380 -   0 255   0
             213  328 -   0 255   0
             213  291 -   0 255   0
             213  272 -   0 255   0
             217  224 -   0   0 231
             218  202 -   0   0 231
             218  135 -   0 255   0
             222   69 -   0 255   0
             225   40 -   0 255   0
             225   11 -  89 124 149
             223    5 -  89 124 149
        "#,
    )
    .await?;

    from_main(move || {
        view.view.place().min_height(250).center_y();
    })
    .await;

    check_colors(
        r#"
             259  449 -  89 124 149
             259  450 -  89 124 149
             252  406 -   0 255   0
             246  388 -   0 255   0
             241  360 -   0 255   0
             233  322 -   0   0 231
             219  251 -   0   0 231
             222  195 -   0   0 231
             220  139 -   0   0 231
             218  101 -   0   0 231
             218   61 -   0 255   0
             223   28 -   0 255   0
             225   15 -  89 124 149
             226    7 -  89 124 149
             504  222 -  89 124 149
             442  220 -  89 124 149
             377  219 -   0 255   0
             289  215 -   0   0 231
             250  214 -   0   0 231
             188  212 -   0   0 231
             157  212 -   0   0 231
             140  214 -   0   0 231
              89  213 -   0 255   0
              75  213 -   0 255   0
              53  217 -   0 255   0
              27  220 -   0 255   0
               8  223 -  89 124 149
               5  223 -  89 124 149
        "#,
    )
    .await?;

    Ok(())
}
