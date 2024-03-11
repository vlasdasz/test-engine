use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    refs::Weak,
    ui::{view, Anchor, SubView, TextField, ViewData, ViewSetup},
    ui_test::{helpers::check_colors, inject_keys, inject_touches},
    App,
};

#[view]
struct TextFieldTestView {
    field:      SubView<TextField>,
    smol_field: SubView<TextField>,
}

impl ViewSetup for TextFieldTestView {
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
    let mut view = App::init_test_view::<TextFieldTestView>(800, 800).await;

    inject_touches(
        r#"
            389  576  b
            389  576  e
            399  292  b
            399  292  e
            427  147  b
            427  147  e
            391  237  b
            391  235  e
    "#,
    )
    .await;

    inject_keys("HELLOY").await;

    inject_touches(
        r#"
            452  442  b
    "#,
    )
    .await;

    inject_keys("text").await;

    inject_touches(
        r#"
            10  10  b
    "#,
    )
    .await;

    check_colors(
        r#"
             328  232 - 203 203 203
             342  232 -   9   9   9
             346  232 -  19  19  19
             357  232 -  82  82  82
             358  232 - 203 203 203
             379  224 - 203 203 203
             393  224 - 203 203 203
             399  224 - 203 203 203
             399  224 - 203 203 203
             410  227 - 203 203 203
             421  227 -   0   0   0
             426  230 - 203 203 203
             434  230 - 203 203 203
             444  229 - 203 203 203
             447  229 - 190 190 190
             452  229 - 203 203 203
             461  229 - 203 203 203
             432  395 - 203 203 203
             412  395 -   0   0   0
             401  395 - 203 203 203
             391  395 - 203 203 203
             378  395 -   0   0   0
             372  395 - 203 203 203
             359  395 - 203 203 203
             388  394 -   0   0   0
             420  394 -   0   0   0
             420  394 -   0   0   0
             381  400 - 203 203 203
             396  400 -   0   0   0
             397  400 -   0   0   0
             409  400 -   0   0   0
             344  224 - 203 203 203
             377  224 - 203 203 203
             397  224 - 203 203 203
             441  235 - 203 203 203
    "#,
    )
    .await?;

    from_main(move || {
        view.field.set_text_size(140);
        view.field.clear();
    })
    .await;

    inject_touches(
        r#"
            452  442  b
    "#,
    )
    .await;

    inject_keys("ŽĖЎФЪ").await;

    check_colors(
        r#"
             146  319 - 128 128 128
             189  319 -  84  84  84
             208  319 - 128 128 128
             250  319 - 128 128 128
             261  319 - 128 128 128
             280  319 -   0   0   0
             341  319 - 128 128 128
             375  316 - 128 128 128
             412  316 - 128 128 128
             512  300 - 128 128 128
             545  303 - 128 128 128
             625  368 - 128 128 128
             664  398 - 128 128 128
             625  404 - 128 128 128
             591  405 - 128 128 128
             575  402 -   0   0   0
             526  387 -   0   0   0
             536  387 - 128 128 128
             506  390 - 128 128 128
             478  393 -   0   0   0
             454  390 - 128 128 128
             454  420 -   0   0   0
             454  434 - 128 128 128
             410  429 - 128 128 128
             391  381 -   0   0   0
             351  367 -   0   0   0
             342  397 - 128 128 128
             292  402 - 128 128 128
             293  384 -   0   0   0
             288  366 - 128 128 128
             260  364 - 128 128 128
             210  367 -   0   0   0
             170  385 - 128 128 128
             159  393 - 128 128 128
             182  408 - 128 128 128
             195  419 - 128 128 128
             316  437 -  25  25  25
             374  402 -   0   0   0
             405  362 -   0   0   0

    "#,
    )
    .await?;

    debug!("Text field test: OK");

    Ok(())
}
