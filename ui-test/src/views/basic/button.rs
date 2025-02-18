use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{Button, HasText, Setup, UI, ViewData, view},
    ui_test::{check_colors, state::increment_state, test_combinations},
};

#[view]
struct ButtonPress {
    #[init]
    button: Button,
}

impl Setup for ButtonPress {
    fn setup(mut self: Weak<Self>) {
        self.button.place().back().size(100, 50).t(25).l(50);
        self.button.set_text("Button text");

        self.button.on_tap(|| {
            increment_state();
        });
    }
}

pub async fn test_button() -> Result<()> {
    UI::init_test_view::<ButtonPress>().await;

    check_colors(
        r#"
              34   46 -  89 124 149
              46   40 -  89 124 149
              60   40 - 255 255 255
              76   42 - 255 255 255
              89   44 -   0   0   0
             121   45 - 255 255 255
             145   45 - 255 255 255
             164   44 -  89 124 149
             162   32 -  89 124 149
             140   13 -  89 124 149
             120    9 -  89 124 149
             111   21 -  89 124 149
             104   38 - 170 170 170
             101   57 - 253 253 253
              97   92 -  89 124 149
              91  114 -  89 124 149
        "#,
    )
    .await?;

    test_combinations([
        ("0 0 b", 0),
        ("0 0 e", 0),
        // Begin inside end outside
        ("100 50 b", 0),
        ("  0 50 e", 0),
        // Begin inside end outside
        ("100 50 b", 0),
        ("  0 50 e", 0),
        // Simple tap
        (
            r"
                100 50 b
                100 50 e
            ",
            1,
        ),
        // Simple tap
        (
            r"
                 90 50 b
                110 50 e
            ",
            1,
        ),
        // Outside then inside
        (
            r"
                  0 50 b
                110 50 e
            ",
            0,
        ),
        // Double release
        (
            r"
                 90 50 b
                110 50 e
                110 50 e
            ",
            1,
        ),
        (
            r"
                23.070313    49.19922     b
                85.86719     52.152344    e
                90.83594     12.671875    b
                89.625       49.941406    e
                184.75781    52.878906    b
                114.35547    48.38672     e
                101.80469    90.75391     b
                105.99219    49.027344    e
            ",
            0,
        ),
        (
            r"
                98.61328     48.339844    b
                0            0            m
                105.02344    50.539063    e

                0            0            m
                102.80469    49.39453     b
                0            0            m
                100.80078    47.55078     e

                0            0            m
                85.49219     50.351563    b
                0            0            m
                99.02734     49.777344    e
                ",
            3,
        ),
        (
            r"
                55.597656    32.632813    b
                55.660156    32.628906    e
                145.63281    33.753906    b
                145.33594    33.8125      e
                144.26172    73.14844     b
                144.19531    73.14844     e
                56.67578     72.02734     b
                56.632813    72.02734     e
                102.44531    50.621094    b
                102.37891    50.621094    e
                172.52344    49.304688    b
                171.8711     49.53125     e
                102.65234    92.15625     b
                102.19141    92.19141     e
                12.4140625   46.382813    b
                12.441406    46.382813    e
                102.51953    16.371094    b
                102.45703    16.199219    e
                ",
            5,
        ),
    ])
    .await?;

    Ok(())
}
