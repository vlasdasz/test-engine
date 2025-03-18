use anyhow::Result;
use test_engine::{
    from_main,
    gm::Apply,
    refs::Weak,
    ui::{DropDown, Setup, UI, ViewData, view},
    ui_test::{
        check_colors, inject_touches, inject_touches_delayed,
        state::{append_state, get_state},
    },
};

#[view]
struct DropDownTestView {
    #[init]
    top: DropDown<&'static str>,
    bot: DropDown<&'static str>,
}

impl Setup for DropDownTestView {
    fn setup(mut self: Weak<Self>) {
        [self.top, self.bot].apply(|v| {
            v.on_changed(|val| {
                append_state(&format!("{val}\n"));
            });
            v.place().size(200, 40).center_x();
        });

        self.top.place().t(5);
        self.bot.place().b(5);

        self.top.set_values(vec!["Dog", "Cat", "Sheep"]);
        self.bot.set_values(vec!["Car", "Boat", "Plane"]);
    }
}

pub async fn test_drop_down() -> Result<()> {
    let mut view = UI::init_test_view::<DropDownTestView>().await;

    assert_eq!(view.top.value(), &"Dog");
    assert_eq!(view.bot.value(), &"Car");

    inject_touches_delayed(
        r"
            334  35   b
            334  35   e
            322  68   b
            321  68   e
            352  585  b
            352  585  e
            326  536  b
            326  536  e
        ",
    )
    .await;

    assert_eq!(view.top.value(), &"Cat");
    assert_eq!(view.bot.value(), &"Boat");

    inject_touches_delayed(
        r"
            363  31   b
            363  31   e
            318  105  b
            318  106  e
            355  580  b
            355  580  e
            343  580  b
            343  579  e
        ",
    )
    .await;

    assert_eq!(view.top.value(), &"Sheep");
    assert_eq!(view.bot.value(), &"Plane");

    inject_touches(
        r"
            342  29   b
            343  29   e
            325  30   b
            325  30   e
            347  575  b
            346  574  e
            345  497  b
            345  497  e
        ",
    )
    .await;

    assert_eq!(view.top.value(), &"Dog");
    assert_eq!(view.bot.value(), &"Car");

    assert_eq!(
        get_state::<String>(),
        r"Cat
Boat
Sheep
Plane
Dog
Car
"
    );

    from_main(move || {
        view.top.custom_format(|val| format!("{val} 5"));
    })
    .await;

    inject_touches(
        "
            228  32   b
            228  32   e

        ",
    )
    .await;

    check_colors(
        r#"
             306  140 -  89 124 149
             306  140 -  89 124 149
             319  136 -  89 124 149
             331  131 -  89 124 149
             338  124 - 255 255 255
             338  117 - 255 255 255
             339  106 - 255 255 255
             343  102 - 235 235 235
             353   98 - 255 255 255
             354   97 - 255 255 255
             358   94 - 255 255 255
             375   89 - 255 255 255
             391   78 - 255 255 255
             403   72 -  89 124 149
             349   62 - 255 255 255
             344   61 - 255 255 255
             337   59 - 255 255 255
             335   59 - 255 255 255
             335   58 - 255 255 255
             332   58 - 255 255 255
             327   57 - 255 255 255
             326   57 - 255 255 255
             324   57 -   0   0   0
             317   56 - 255 255 255
             310   57 - 122 122 122
             306   58 -   0   0   0
             303   58 -  42  42  42
             289   57 - 188 188 188
             287   57 - 255 255 255
             282   57 - 174 174 174
             282   56 -  59  59  59
             259   57 - 255 255 255
             330   11 - 210 210 210
             329   12 -   0   0   0
             326   13 - 255 255 255
             317   13 - 255 255 255
             319   16 - 255 255 255
             329   17 -  13  13  13
             333   16 - 255 255 255
             343   16 - 255 255 255
             352   15 - 255 255 255
             364   16 - 255 255 255
        "#,
    )
    .await?;

    Ok(())
}
