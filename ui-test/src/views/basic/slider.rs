use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    refs::Weak,
    ui::{view, Anchor, Color, Label, Slider, ViewData, ViewFrame, ViewSetup, ViewSubviews, UI},
    ui_test::{helpers::check_colors, inject_touches},
};

#[view]
pub struct SliderTestView {
    #[init]
    slider: Slider,
    label:  Label,
}

impl ViewSetup for SliderTestView {
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
    let mut view = UI::init_test_view::<SliderTestView>().await;

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
        r"
             357  503 -  25  51  76
             357  497 -  25  51  76
             360  490 -  25  51  76
             365  471 -   0   0   0
             362  461 -  25  51  76
             362  441 -  61  61  61
             363  410 -  35  35  35
             364  396 -   1   1   1
             362  382 -  25  51  76
             362  374 - 255 255 255
             362  364 - 255 255 255
             362  330 - 255 255 255
             362  321 -  25  51  76
             364  311 -  25  51  76
             363  296 - 255 255 255
             363  275 -  25  51  76
             365  252 -  25  51  76
             365  226 - 255 255 255
             370  181 -  25  51  76
             370  156 - 255 255 255
             369  145 -  25  51  76
             366  131 -  94  94  94
             360  115 -   0   0   0
             327   86 -  25  51  76
             293   90 -  25  51  76
             281  121 -   0   0 203
             343  128 - 255 255 255
             356  128 - 255 255 255
             380  128 - 255 255 255
             367  153 -   1   1   1
             375  203 - 255 255 255
             373  204 - 255 255 255
             375  229 - 255 255 255
             374  258 - 255 255 255
             372  269 - 255 255 255
             374  297 - 255 255 255
             373  323 -  25  51  76
             368  349 -  25  51  76
             367  365 - 255 255 255
             368  411 - 132 132 132
             369  465 - 255 255 255
             368  482 -   1   1   1
             369  501 -  25  51  76
        ",
    )
    .await?;

    debug!("Slider test: OK");

    Ok(())
}
