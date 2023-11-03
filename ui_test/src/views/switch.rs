use anyhow::Result;
use log::debug;
use test_engine::Screen;
use ui::{refs::Weak, view, SubView, ViewSetup};
use ui_views::Switch;

use crate::view_tests::{state::set_state, test_combinations};

#[view]
struct SwitchTestView {
    switch: SubView<Switch>,
}

impl ViewSetup for SwitchTestView {
    fn setup(self: Weak<Self>) {
        self.switch.place.back().size(100, 50).center();

        self.switch.selected.val(|on| {
            set_state(on);
        });
    }
}

pub async fn test_switch() -> Result<()> {
    Screen::set_test_view::<SwitchTestView>().await;

    test_combinations([
        (
            r#"
                174.58594    49.171875    ↓
                114.09766    45.835938    ↑
                98.78125     10.671875    ↓
                95.50391     49.164063    ↑
                18.003906    50.20703     ↓
                66.41797     48.191406    ↑
                119.44531    86.00391     ↓
                118.953125   47.95703     ↑
                "#,
            false,
        ),
        (
            r#"
                56.40625     35.191406    ↓
                56.40625     35.1875      ↑
                141.73047    37.035156    ↓
                141.66406    37.035156    ↑
                140.44531    69.25        ↓
                140.3789     69.25        ↑
                56.01172     69.88672     ↓
                56.04297     69.88672     ↑
                100.87109    50.507813    ↓
                100.80469    50.507813    ↑
                "#,
            true,
        ),
        (
            r#"
                98.99219     54.15625     ↓
                98.99219     54.15625     ↑
                98.99219     54.15625     ↓
                98.99219     54.15625     ↑
                "#,
            true,
        ),
        (
            r#"
                98.99219     54.15625     ↓
                98.99219     54.15625     ↑
                "#,
            false,
        ),
    ])
    .await?;

    debug!("Switch test: OK");

    Ok(())
}
