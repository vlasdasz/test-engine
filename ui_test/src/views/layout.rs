use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, Anchor, Button, Color, SubView, ViewData, ViewSetup, ViewSubviews},
    App,
};

use crate::view_tests::{
    assert_eq, inject_touches,
    state::{append_state, clear_state, get_state},
};

#[view]
struct LayoutTestView {
    #[text = center]
    center: SubView<Button>,

    #[text = top]
    top:    SubView<Button>,
    #[text = bottom]
    bottom: SubView<Button>,
    #[text = left]
    left:   SubView<Button>,
    #[text = right]
    right:  SubView<Button>,

    #[text = tp_ct]
    top_center:    SubView<Button>,
    #[text = bt_ct]
    bottom_center: SubView<Button>,
    #[text = le_ct]
    left_center:   SubView<Button>,
    #[text = ri_ct]
    right_center:  SubView<Button>,

    #[text = to_s_ct]
    top_s_center:    SubView<Button>,
    #[text = bo_s_ct]
    bottom_s_center: SubView<Button>,
    #[text = le_s_ct]
    left_s_center:   SubView<Button>,
    #[text = ri_s_ct]
    right_s_center:  SubView<Button>,
}

impl ViewSetup for LayoutTestView {
    fn setup(mut self: Weak<Self>) {
        for view in self.subviews_mut() {
            view.place().size(15, 15);

            let Some(button) = view.downcast::<Button>() else {
                continue;
            };

            button.on_tap(move || {
                append_state(&format!("|{}", button.text()));
            })
        }

        self.center.place().center();

        self.top.set_color(Color::ORANGE).place().center_x().t(80);
        self.bottom.set_color(Color::GREEN).place().center_x().b(80);
        self.left.place().center_y().l(80);
        self.right.place().center_y().r(80);

        self.top_center.place().between(self.top, self.center);
        self.bottom_center.place().between(self.bottom, self.center);
        self.left_center.place().between(self.left, self.center);
        self.right_center.place().between(self.right, self.center);

        self.top_s_center.place().between_super(self.top, Anchor::Top);
        self.bottom_s_center.place().between_super(self.bottom, Anchor::Bot);
        self.left_s_center.place().between_super(self.left, Anchor::Left);
        self.right_s_center.place().between_super(self.right, Anchor::Right);
    }
}

pub async fn test_layout() -> Result<()> {
    App::init_test_view::<LayoutTestView>(240, 240).await;

    inject_touches(
        r#"
            8    121  b
            9    121  e
            25   122  b
            25   122  e
            40   122  b
            40   122  e
            51   122  b
            51   122  e
            73   122  b
            74   122  e
            84   120  b
            84   120  e
            101  121  b
            101  121  e
            120  122  b
            120  122  e
            134  122  b
            134  121  e
            154  121  b
            154  121  e
            164  122  b
            164  122  e
            182  122  b
            182  122  e
            200  122  b
            200  122  e
            222  121  b
            222  121  e
            120  232  b
            120  231  e
            121  221  b
            121  220  e
            122  204  b
            122  204  e
            119  189  b
            120  189  e
            121  169  b
            121  169  e
            119  151  b
            119  151  e
            119  139  b
            119  139  e
            120  121  b
            120  121  e
            122  108  b
            122  108  e
            119  88   b
            119  88   e
            120  75   b
            120  75   e
            121  58   b
            121  58   e
            119  43   b
            119  43   e
            118  18   b
            118  18   e
            "#,
    )
    .await;

    assert_eq(
        get_state::<String>(),
        "|le_s_ct|left|le_ct|center|ri_ct|right|ri_s_ct|bo_s_ct|bottom|bt_ct|center|tp_ct|top|to_s_ct",
    )?;

    App::set_window_size((400, 400));

    clear_state();

    inject_touches(
        r#"
            4    201  b
            5    201  e
            27   201  b
            27   201  e
            41   201  b
            41   201  e
            52   202  b
            53   201  e
            70   202  b
            73   202  e
            87   202  b
            87   202  e
            101  202  b
            101  202  e
            129  201  b
            129  201  e
            146  201  b
            146  201  e
            157  202  b
            157  202  e
            189  204  b
            189  204  e
            199  203  b
            200  203  e
            218  204  b
            217  204  e
            243  204  b
            243  204  e
            259  204  b
            259  204  e
            271  204  b
            271  204  e
            301  203  b
            302  203  e
            315  204  b
            315  204  e
            326  201  b
            326  202  e
            344  203  b
            345  203  e
            359  203  b
            359  203  e
            382  202  b
            382  201  e
            198  396  b
            198  396  e
            202  380  b
            202  379  e
            202  360  b
            202  361  e
            201  349  b
            201  349  e
            203  324  b
            203  323  e
            201  312  b
            201  312  e
            200  300  b
            200  300  e
            201  269  b
            201  268  e
            201  258  b
            201  258  e
            201  244  b
            200  244  e
            200  220  b
            200  220  e
            201  201  b
            201  201  e
            201  190  b
            201  190  e
            202  160  b
            202  160  e
            201  148  b
            201  148  e
            200  130  b
            200  130  e
            200  104  b
            200  104  e
            199  86   b
            199  86   e
            200  73   b
            200  73   e
            200  57   b
            200  57   e
            201  41   b
            201  41   e
            200  30   b
            200  30   e
            200  11   b
            200  11   e
            "#,
    )
    .await;

    assert_eq(
        get_state::<String>(),
        "|le_s_ct|left|le_ct|center|ri_ct|right|ri_s_ct|bo_s_ct|bottom|bt_ct|center|tp_ct|top|to_s_ct",
    )?;

    debug!("Layout test: OK");
    Ok(())
}
