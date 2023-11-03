use anyhow::Result;
use log::debug;
use test_engine::{from_main, gm::Color, rtools::Apply, Screen};
use ui::{layout::Anchor, refs::Weak, view, SubView, ViewData, ViewSetup, ViewSubviews};
use ui_views::Button;

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
            view.place.size(50, 50);

            let Some(button) = view.downcast::<Button>() else {
                continue;
            };

            button.on_tap(move || {
                append_state(&format!("_{}", button.text()));
            })
        }

        [self.center, self.top, self.bottom, self.left, self.right].apply(|view| {
            view.place.size(100, 100);
        });

        self.center.place.center();

        self.top.set_color(Color::ORANGE).place.center_x().t(200);
        self.bottom.set_color(Color::GREEN).place.center_x().b(200);
        self.left.place.center_y().l(200);
        self.right.place.center_y().r(200);

        self.top_center.place.between(self.top, self.center);
        self.bottom_center.place.between(self.bottom, self.center);
        self.left_center.place.between(self.left, self.center);
        self.right_center.place.between(self.right, self.center);

        self.top_s_center.place.between_super(self.top, Anchor::Top);
        self.bottom_s_center.place.between_super(self.bottom, Anchor::Bot);
        self.left_s_center.place.between_super(self.left, Anchor::Left);
        self.right_s_center.place.between_super(self.right, Anchor::Right);
    }
}

pub async fn test_layout() -> Result<()> {
    Screen::set_test_view::<LayoutTestView>(850, 850).await;

    inject_touches(
        r#"
            104.609375   424.64453    b
            104.609375   424.64453    e
            234.04297    430.95703    b
            234.04297    430.95703    e
            334.96094    426.4336     b
            334.96094    426.1914     e
            430.1172     429.29688    b
            430.1172     429.29688    e
            521.7422     426.41406    b
            521.7422     426.41406    e
            588.08203    426.13672    b
            588.08203    426.13672    e
            743.0469     428.61328    b
            743.0469     428.61328    e
            427.14844    754.0625     b
            427.14844    754.0625     e
            429.39844    588.0039     b
            429.39844    587.7617     e
            426.92188    516.4336     b
            426.92188    516.1836     e
            425.40234    334.46094    b
            425.40234    334.46094    e
            430.125      217.41406    b
            430.125      217.17578    e
            426.8711     88.13281     b
            426.8711     88.13281     e
            "#,
    )
    .await;

    assert_eq(
        get_state::<String>(),
        "_le_s_ct_left_le_ct_center_ri_ct_right_ri_s_ct_bo_s_ct_bottom_bt_ct_tp_ct_top_to_s_ct",
    )?;

    from_main(|| {
        Screen::current().set_size((1600, 1200));
    })
    .await;

    clear_state();

    inject_touches(
        r#"
            1075.1445    4.1875       b
            890.71875    -33.57422    e
            110.32422    606.8711     b
            110.98828    606.84375    e
            242.82031    602.91406    b
            244.70313    602.9961     e
            532.3711     603.89453    b
            533.64453    603.8867     e
            815.5508     607.58594    b
            820.08984    607.46484    e
            1089.2617    600.9375     b
            1089.3555    600.96484    e
            1356.9961    596.83203    b
            1363.875     597.25       e
            1504.6953    605.35547    b
            1504.7852    605.3203     e
            795.8008     1101.8008    b
            795.6836     1101.9883    e
            822.03906    929.2344     b
            822.03516    929.2344     e
            799.3711     779.51953    b
            799.4961     779.9297     e
            813.9492     617.8008     b
            813.8711     617.03906    e
            796.53125    428.84375    b
            796.53125    428.84375    e
            802.1797     272.11328    b
            802.1797     272.07813    e
            800.39453    99.57422     b
            800.3281     99.57422     e
            "#,
    )
    .await;

    assert_eq(
        get_state::<String>(),
        "_le_s_ct_left_le_ct_center_ri_ct_right_ri_s_ct_bo_s_ct_bottom_bt_ct_center_tp_ct_top_to_s_ct",
    )?;

    debug!("Layout test: OK");
    Ok(())
}
