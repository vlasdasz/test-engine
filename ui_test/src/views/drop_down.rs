use anyhow::Result;
use log::debug;
use test_engine::{
    gm::Apply,
    refs::Weak,
    ui::{view, DropDown, Sub, ViewData, ViewSetup},
    ui_test::{
        inject_touches, inject_touches_delayed,
        state::{append_state, clear_state, get_state},
    },
    App,
};

#[view]
struct DropDownTestView {
    top: Sub<DropDown>,
    bot: Sub<DropDown>,
}

impl ViewSetup for DropDownTestView {
    fn setup(mut self: Weak<Self>) {
        [self.top, self.bot].apply(|v| {
            v.on_changed(|val| {
                append_state(&format!("{val}\n"));
            });
            v.place().size(200, 40).center_x();
        });

        self.top.place().t(5);
        self.bot.place().b(5);

        self.top.set_values(["Dog", "Cat", "Sheep"]);
        self.bot.set_values(["Car", "Boat", "Plane"]);
    }
}

pub async fn test_drop_down() -> Result<()> {
    let view = App::init_test_view::<DropDownTestView>().await;

    clear_state();

    assert_eq!(view.top.text(), "Dog");
    assert_eq!(view.bot.text(), "Car");

    inject_touches_delayed(
        r#"
            334  35   b
            334  35   e
            322  68   b
            321  68   e
            352  585  b
            352  585  e
            326  536  b
            326  536  e
        "#,
    )
    .await;

    assert_eq!(view.top.text(), "Cat");
    assert_eq!(view.bot.text(), "Boat");

    inject_touches_delayed(
        r#"
            363  31   b
            363  31   e
            318  105  b
            318  106  e
            355  580  b
            355  580  e
            343  580  b
            343  579  e
        "#,
    )
    .await;

    assert_eq!(view.top.text(), "Sheep");
    assert_eq!(view.bot.text(), "Plane");

    inject_touches(
        r#"
            342  29   b
            343  29   e
            325  30   b
            325  30   e
            347  575  b
            346  574  e
            345  497  b
            345  497  e
        "#,
    )
    .await;

    assert_eq!(view.top.text(), "Dog");
    assert_eq!(view.bot.text(), "Car");

    assert_eq!(
        get_state::<String>(),
        r#"Cat
Boat
Dog
Sheep
Plane
Plane
Dog
Dog
Plane
Car
"#
    );

    debug!("Drop down test: OK");

    Ok(())
}
