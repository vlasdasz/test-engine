#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(process_exitcode_internals)]

use std::process::ExitCode;

use test_engine::gm::flat::IntSize;
use ui::{refs::Weak, view, SubView, ViewSetup, ViewTest};
use ui_views::Switch;

use crate::view_tests::{state::set_state, test_combinations};

mod view_tests;

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

impl ViewTest for SwitchTestView {
    fn test_size() -> IntSize
    where Self: Sized {
        (200, 100).into()
    }
}

fn main() -> ExitCode {
    test_engine::ViewApp::<SwitchTestView>::start_with_actor(async {
        // return crate::view_tests::record_touches().await;
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
        ]);
    })
}
