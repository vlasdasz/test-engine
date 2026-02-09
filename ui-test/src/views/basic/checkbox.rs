use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{CheckBox, Setup, ViewFrame, view},
    ui_test::{UITest, inject_touches},
};

#[view]
struct CheckBoxTestView {
    #[init]
    checkbox: CheckBox,
}

impl Setup for CheckBoxTestView {
    fn setup(self: Weak<Self>) {
        self.checkbox.set_frame((50, 50, 50, 50));
    }
}

pub async fn test_checkbox() -> Result<()> {
    let view = UITest::start::<CheckBoxTestView>();

    assert!(!view.checkbox.on());

    inject_touches(
        "
         81   86   b
         81   86   e

     ",
    );

    assert!(view.checkbox.on());

    inject_touches(
        "
         81   86   b
         81   86   e

     ",
    );

    assert!(!view.checkbox.on());

    Ok(())
}
