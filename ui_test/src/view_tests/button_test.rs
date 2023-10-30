use std::process::ExitCode;

use test_engine::gm::flat::IntSize;
use ui::{refs::Weak, view, SubView, ViewSetup, ViewTest};
use ui_views::Button;

use crate::view_tests::{state::increment_state, test_combinations};

#[view]
struct ButtonTestView {
    button: SubView<Button>,
}

impl ViewSetup for ButtonTestView {
    fn setup(mut self: Weak<Self>) {
        self.button.place.back().size(100, 50).center();
        self.button.set_text("Button text");

        self.button.on_tap.sub(|| {
            increment_state();
        });
    }
}

impl ViewTest for ButtonTestView {
    fn test_size() -> IntSize
    where Self: Sized {
        (200, 100).into()
    }
}

#[allow(dead_code)]
pub fn test_button_view() -> ExitCode {
    test_engine::ViewApp::<ButtonTestView>::start_with_actor(async {
        // return record_touches().await;
        test_combinations([
            ("0 0 ↓", 0),
            ("0 0 ↑", 0),
            // Begin inside end outside
            ("100 50 ↓", 0),
            ("  0 50 ↑", 0),
            // Begin inside end outside
            ("100 50 ↓", 0),
            ("  0 50 ↑", 0),
            // Simple tap
            (
                r#"
                100 50 ↓
                100 50 ↑
            "#,
                1,
            ),
            // Simple tap
            (
                r#"
                 90 50 ↓
                110 50 ↑
            "#,
                1,
            ),
            // Double release
            (
                r#"
                 90 50 ↓
                110 50 ↑
                110 50 ↑
            "#,
                1,
            ),
            // Outside then inside
            (
                r#"
                  0 50 ↓
                110 50 ↑
            "#,
                0,
            ),
            (
                r#"
                23.070313    49.19922     ↓
                85.86719     52.152344    ↑
                90.83594     12.671875    ↓
                89.625       49.941406    ↑
                184.75781    52.878906    ↓
                114.35547    48.38672     ↑
                101.80469    90.75391     ↓
                105.99219    49.027344    ↑
            "#,
                0,
            ),
            (
                r#"
                98.61328     48.339844    ↓
                0            0            →
                105.02344    50.539063    ↑
            
                0            0            →
                102.80469    49.39453     ↓
                0            0            →
                100.80078    47.55078     ↑
            
                0            0            →
                85.49219     50.351563    ↓
                0            0            →
                99.02734     49.777344    ↑
                "#,
                3,
            ),
            (
                r#"
                55.597656    32.632813    ↓
                55.660156    32.628906    ↑
                145.63281    33.753906    ↓
                145.33594    33.8125      ↑
                144.26172    73.14844     ↓
                144.19531    73.14844     ↑
                56.67578     72.02734     ↓
                56.632813    72.02734     ↑
                102.44531    50.621094    ↓
                102.37891    50.621094    ↑
                172.52344    49.304688    ↓
                171.8711     49.53125     ↑
                102.65234    92.15625     ↓
                102.19141    92.19141     ↑
                12.4140625   46.382813    ↓
                12.441406    46.382813    ↑
                102.51953    16.371094    ↓
                102.45703    16.199219    ↑
                "#,
                5,
            ),
        ]);
    })
}
