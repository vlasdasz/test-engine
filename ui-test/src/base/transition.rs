use anyhow::Result;
use parking_lot::Mutex;
use test_engine::{
    refs::Weak,
    ui::{BLUE, Button, HasText, Setup, UI, ViewData, ViewTransition, view},
    ui_test::{check_colors, inject_touches},
};

static ACTIONS: Mutex<Vec<&str>> = Mutex::new(vec![]);

#[view]
struct Transition {
    #[init]
    to_blue: Button,
}

impl Setup for Transition {
    fn setup(mut self: Weak<Self>) {
        self.to_blue.set_text("To Blue");
        self.to_blue.place().tl(20).size(200, 100);
        self.to_blue.add_transition::<Self, BlueView>();
    }
}

impl ViewTransition<BlueView> for Transition {
    fn transition_to(self: Weak<Self>, _target: &mut BlueView) {
        ACTIONS.lock().push("Transition callback");
    }
}

#[view]
struct BlueView {}

impl Setup for BlueView {
    fn setup(mut self: Weak<Self>) {
        self.set_color(BLUE);
        ACTIONS.lock().push("Blue setup");
    }
}

pub async fn test_transition() -> Result<()> {
    UI::init_test_view::<Transition>();

    check_colors(
        r#"
             117  382 -  89 124 149
             247  320 -  89 124 149
             361  222 -  89 124 149
             487   78 -  89 124 149
        "#,
    )?;

    inject_touches(
        "
            142  88   b
            142  87   e

        ",
    );

    check_colors(
        r#"
             103  384 -   0   0 231
             221  301 -   0   0 231
             430  120 -   0   0 231
             559   52 -   0   0 231
        "#,
    )?;

    assert_eq!(ACTIONS.lock().as_slice(), &["Transition callback", "Blue setup"]);

    ACTIONS.lock().clear();

    Ok(())
}
