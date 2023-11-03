use anyhow::Result;
use log::debug;
use test_engine::{gm::flat::IntSize, Screen};
use ui::{refs::Weak, view, SubView, TouchStack, View, ViewSetup, ViewSubviews, ViewTest};
use ui_views::{Button, DropDown};

use crate::view_tests::record_touches;

#[view]
struct DropDownTestView {
    top: SubView<DropDown>,
    bot: SubView<DropDown>,
}

fn add_test_button(mut view: Weak<dyn View>, mut action: impl FnMut() + 'static) {
    let mut button = view.add_view::<Button>();
    button.set_text("TAP").place.size(100, 20).center();
    button.on_tap.sub(move || action())
}

impl ViewSetup for DropDownTestView {
    fn setup(mut self: Weak<Self>) {
        self.top.place.size(100, 28).center_x().t(5);
        self.bot.place.size(100, 28).center_x().b(5);

        self.top.set_values(["Dog", "Cat", "Sheep"]);
        self.bot.set_values(["Car", "Boat", "Plane"]);

        add_test_button(self, || {
            println!("{}", TouchStack::dump());
        })
    }
}

impl ViewTest for DropDownTestView {
    fn test_size() -> IntSize
    where Self: Sized {
        (280, 280).into()
    }
}

pub async fn test_drop_down() -> Result<()> {
    Screen::set_test_view::<DropDownTestView>().await;

    record_touches().await;

    debug!("Drop down test: OK");

    Ok(())
}
