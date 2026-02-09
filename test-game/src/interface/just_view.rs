use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{BLUE, Button, RED, Setup, ViewData, ViewTest, view_test},
    ui_test::check_colors,
};

#[view_test]
struct JustView {
    #[init]
    red:  Button,
    blue: Button,
}

impl Setup for JustView {
    fn setup(self: Weak<Self>) {
        self.red.set_color(RED).place().left_half();
        self.blue.set_color(BLUE).place().right_half();
    }
}

impl ViewTest for JustView {
    fn perform_test(_: Weak<Self>) -> Result<()> {
        check_colors(
            r"
                     497   95 -   0   0 231
                     386  105 -   0   0 231
                     259  111 - 255   0   0
                      93  110 - 255   0   0
                ",
        )?;

        Ok(())
    }
}
