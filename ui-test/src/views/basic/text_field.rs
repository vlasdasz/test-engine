use anyhow::Result;
use test_engine::{
    App, from_main,
    refs::Weak,
    ui::{Anchor, Setup, UI, ViewData, view},
    ui_test::{helpers::check_colors, inject_keys, inject_touches},
};

#[view]
struct TextField {
    #[init]
    field:      test_engine::ui::TextField,
    smol_field: test_engine::ui::TextField,
}

impl Setup for TextField {
    fn setup(self: Weak<Self>) {
        self.field.place().size(600, 200).center();
        self.smol_field
            .place()
            .size(200, 50)
            .center_x()
            .anchor(Anchor::Bot, self.field, 40);
    }
}

pub async fn test_text_field() -> Result<()> {
    let mut view = UI::init_test_view::<TextField>().await;

    App::set_window_size((800, 800)).await;

    inject_touches(
        r"
            389  576  b
            389  576  e
            399  292  b
            399  292  e
            427  147  b
            427  147  e
            391  237  b
            391  235  e
    ",
    )
    .await;

    inject_keys("HELLOY").await;

    inject_touches(
        r"
            452  442  b
    ",
    )
    .await;

    inject_keys("text").await;

    inject_touches(
        r"
            10  10  b
    ",
    )
    .await;

    check_colors(
        r#"
             357  529 -  89 124 149
             368  511 -  89 124 149
             375  483 - 231 231 231
             385  458 - 231 231 231
             393  426 - 231 231 231
             396  393 -  69  69  69
             398  379 - 231 231 231
             401  390 - 231 231 231
             388  281 -  89 124 149
             380  243 - 231 231 231
             377  242 -   0   0   0
             354  234 - 164 164 164
             331  219 - 231 231 231
             327  218 - 231 231 231
             415  229 - 231 231 231
             424  228 - 231 231 231
             476  228 - 231 231 231
             487  223 - 231 231 231
             490  188 -  89 124 149
             437  182 -  89 124 149
             386  173 -  89 124 149
             328  190 -  89 124 149
             331  242 - 231 231 231
             444  339 - 231 231 231
             538  421 - 231 231 231
             466  472 - 231 231 231
             333  475 - 231 231 231
             272  502 -  89 124 149
             284  560 -  89 124 149
        "#,
    )
    .await?;

    from_main(move || {
        view.field.set_text_size(140);
        view.field.clear();
    })
    .await;

    inject_touches(
        r"
            452  442  b
    ",
    )
    .await;

    inject_keys("ŽĖЎФЪ").await;

    check_colors(
        r#"
              75  403 -  89 124 149
              96  393 -  89 124 149
             117  387 - 188 188 188
             185  381 -  34  34  34
             191  380 -   7   7   7
             193  380 -   7   7   7
             238  370 - 188 188 188
             259  373 - 188 188 188
             275  375 - 188 188 188
             307  378 - 188 188 188
             377  383 - 188 188 188
             401  385 - 188 188 188
             428  385 -   7   7   7
             489  388 - 188 188 188
             529  393 -   7   7   7
             577  392 -   7   7   7
             657  401 - 188 188 188
             713  387 -  89 124 149
             728  293 -  89 124 149
             569  210 -  89 124 149
             477  215 - 231 231 231
             325  234 - 231 231 231
             295  234 -  89 124 149
        "#,
    )
    .await?;

    Ok(())
}
