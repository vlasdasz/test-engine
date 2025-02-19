use anyhow::Result;
use test_engine::{
    from_main,
    refs::Weak,
    ui::{Anchor, Color, HasText, Label, Setup, UI, ViewData, ViewFrame, ViewSubviews, view},
    ui_test::{helpers::check_colors, inject_touches},
};

#[view]
pub struct Slider {
    #[init]
    slider: test_engine::ui::Slider,
    label:  Label,
}

impl Setup for Slider {
    fn setup(mut self: Weak<Self>) {
        self.slider.set_color(Color::WHITE).place().size(50, 400).center();
        self.slider.on_change.val(move |a| {
            self.label.set_text(a);
        });

        self.label
            .place()
            .size(100, 50)
            .center_y()
            .anchor(Anchor::Right, self.slider, 40);
    }
}

pub async fn test_slider() -> Result<()> {
    let mut view = UI::init_test_view::<Slider>().await;

    inject_touches(
        r"
            306  202  b
            306  202  e
    ",
    )
    .await;

    assert_eq!(view.slider.value(), 0.78);
    assert_eq!(view.label.text(), "0.78");

    inject_touches(
        r"
            177  137  m
            183  139  m
            195  138  m
            196  138  b
            197  139  m
            270  148  m
            290  148  m
            307  148  m
            304  160  m
            302  184  m
            301  234  m
            299  268  m
            292  315  m
            288  371  m
            290  409  m
            288  417  m
            195  448  m
            173  455  m
            173  455  e
            172  449  m
    ",
    )
    .await;

    assert_eq!(view.slider.value(), 0.78);
    assert_eq!(view.label.text(), "0.78");

    inject_touches(
        r"
            317  218  m
            303  208  m
            300  205  m
            300  205  b
            300  205  m
            325  208  m
            362  220  m
            378  240  m
            387  261  m
            381  292  m
            364  309  m
            342  324  m
            320  339  m
            299  357  m
            283  372  m
            269  395  m
            269  400  m
            274  420  m
            288  429  m
            298  429  m
            334  431  m
            359  431  m
            371  432  m
            378  433  e
            379  432  m
            465  391  m
            488  356  m
            455  444  m
            389  459  m
            416  449  m
            482  405  m
    ",
    )
    .await;

    assert_eq!(view.slider.value(), 0.122_857_15);
    assert_eq!(view.label.text(), "0.12");

    inject_touches(
        r"
            322  443  m
            312  438  m
            308  437  m
            308  438  b
            306  446  m
            299  464  m
            289  488  m
            273  512  m
            252  531  m
            248  537  m
            247  536  e
            248  535  m
            323  538  m
    ",
    )
    .await;

    assert_eq!(view.slider.value(), 0.0);
    assert_eq!(view.label.text(), "0.00");

    inject_touches(
        r"
            337  478  m
            306  475  m
            297  476  m
            298  477  m
            299  477  b
            322  458  m
            363  379  m
            382  230  m
            359  107  m
            316  46   m
            303  35   m
            304  37   e
            304  38   m
            435  184  m
            469  194  m
            477  177  m
    ",
    )
    .await;

    assert_eq!(view.slider.value(), 1.0);
    assert_eq!(view.label.text(), "1.00");

    from_main(move || {
        view.slider.set_range(-5, 5);
    })
    .await;

    assert_eq!(view.slider.value(), 5.0);
    assert_eq!(view.label.text(), "5.00");

    inject_touches(
        "
            301  136  b
            303  186  m
            307  313  m
            306  446  m
            304  507  m
            303  543  m
            303  542  e
        ",
    )
    .await;

    assert_eq!(view.slider.value(), -5.0);
    assert_eq!(view.label.text(), "-5.00");

    for i in -5..=5 {
        from_main(move || {
            view.slider.set_value(i);
            let mut label = view.add_view::<Label>();
            label.set_text(i);
            label.set_size((50, 20));
            label.set_x(340);
            label.set_y(view.slider.indicator_position() - 10.0 + view.slider.y());
        })
        .await;
    }

    check_colors(
        r#"
             370  507 -  89 124 149
             371  504 -  89 124 149
             369  491 -  89 124 149
             365  484 - 255 255 255
             361  466 - 255 255 255
             361  459 -  89 124 149
             362  440 - 151 151 151
             362  418 -  89 124 149
             362  410 - 252 252 252
             362  390 -  89 124 149
             363  373 - 222 222 222
             363  361 - 153 153 153
             363  354 -  89 124 149
             365  342 - 255 255 255
             365  330 - 255 255 255
             364  313 -  89 124 149
             362  286 -  89 124 149
             364  264 - 176 176 176
             363  238 - 162 162 162
             363  218 -  89 124 149
             363  208 -  89 124 149
             363  188 - 255 255 255
             363  170 -  89 124 149
             366  148 -  89 124 149
             368  145 -  89 124 149
             371  136 -  89 124 149
             372  129 - 254 254 254
             373  113 -  89 124 149
             373  107 -  89 124 149
             373   77 -  89 124 149
        "#,
    )
    .await?;

    Ok(())
}
