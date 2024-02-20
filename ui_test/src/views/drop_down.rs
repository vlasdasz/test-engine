use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, Button, DropDown, SubView, TouchStack, ViewData, ViewSetup, ViewSubviews, WeakView},
    App,
};

use crate::view_tests::assert_eq;

#[view]
struct DropDownTestView {
    top: SubView<DropDown>,
    bot: SubView<DropDown>,
}

fn add_test_button(mut view: WeakView, action: impl FnMut() + 'static) {
    let mut button = view.add_view::<Button>();
    button.set_text("TAP").place().size(100, 20).center();
    button.on_tap(action)
}

impl ViewSetup for DropDownTestView {
    fn setup(mut self: Weak<Self>) {
        self.top.place().size(100, 28).center_x().t(5);
        self.bot.place().size(100, 28).center_x().b(5);

        self.top.set_values(["Dog", "Cat", "Sheep"]);
        self.bot.set_values(["Car", "Boat", "Plane"]);

        add_test_button(self, || {
            println!("{:?}", TouchStack::dump());
        })
    }
}

pub async fn test_drop_down() -> Result<()> {
    let view = App::set_test_view::<DropDownTestView>(280, 280).await;

    assert_eq(view.top.text(), "Dog")?;
    assert_eq(view.bot.text(), "Car")?;

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
