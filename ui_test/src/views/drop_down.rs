use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, DropDown, SubView, TouchStack, ViewData, ViewSetup},
    App,
};

use crate::utils::{assert_eq, helpers::add_action, record_ui_test};

#[view]
struct DropDownTestView {
    top: SubView<DropDown>,
    //  bot: SubView<DropDown>,
}

impl ViewSetup for DropDownTestView {
    fn setup(mut self: Weak<Self>) {
        self.top.place().size(200, 40).center_x().t(5);

        self.top.on_changed(|val| {
            dbg!(&val);
        });

        // self.bot.place().size(200, 40).center_x().b(5);

        self.top.set_values(["Dog", "Cat", "Sheep"]);
        // self.bot.set_values(["Car", "Boat", "Plane"]);

        add_action(|| {
            dbg!(TouchStack::dump());
        });
    }
}

pub async fn test_drop_down() -> Result<()> {
    let view = App::init_test_view::<DropDownTestView>(600, 600).await;

    assert_eq(view.top.text(), "Dog")?;
    // assert_eq(view.bot.text(), "Car")?;

    dbg!(TouchStack::dump());
    //
    // assert_eq(
    //     TouchStack::dump(),
    //     vec![vec![
    //         "Layer: Root view".to_string(),
    //         // "View: ".to_string() + &button.label.clone(),
    //     ]],
    // )?;

    record_ui_test().await?;

    // inject_touches(
    //     r#"
    //         169.54688    28.765625    b
    //         169.54688    28.765625    e
    //         160.89063    46.445313    b
    //         160.89063    46.445313    e
    //         160.4414     265.8789     b
    //         160.4414     265.8789     e
    //         150.78516    230.16406    b
    //         150.78516    230.16406    e
    // "#,
    // )
    // .await;

    // assert_eq(view.top.text(), "Cat")?;
    // assert_eq(view.bot.text(), "Boat")?;

    debug!("Drop down test: OK");

    Ok(())
}
