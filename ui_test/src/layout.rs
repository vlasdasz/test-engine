use anyhow::{bail, Result};
use log::debug;
use test_engine::{
    from_main,
    gm::{flat::IntSize, Color},
    rtools::Apply,
    Screen,
};
use ui::{layout::Anchor, refs::Weak, view, SubView, ViewData, ViewSetup, ViewSubviews, ViewTest};
use ui_views::Button;

use crate::view_tests::{
    inject_touches,
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

            button.on_tap.sub(move || {
                append_state(&format!("_{}", button.text()));
            })
        }

        [self.center, self.top, self.bottom, self.left, self.right].apply(|view| {
            view.place.size(100, 100);
        });

        self.center.place.center();

        self.top.set_color(Color::ORANGE).place.center_hor().t(200);
        self.bottom.set_color(Color::GREEN).place.center_hor().b(200);
        self.left.place.center_ver().l(200);
        self.right.place.center_ver().r(200);

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

impl ViewTest for LayoutTestView {
    fn test_size() -> IntSize
    where Self: Sized {
        (850, 850).into()
    }
}

pub async fn test_layout() -> Result<()> {
    Screen::set_test_view::<LayoutTestView>().await;

    inject_touches(
        r#"
            104.609375   424.64453    ↓
            104.609375   424.64453    ↑
            234.04297    430.95703    ↓
            234.04297    430.95703    ↑
            334.96094    426.4336     ↓
            334.96094    426.1914     ↑
            430.1172     429.29688    ↓
            430.1172     429.29688    ↑
            521.7422     426.41406    ↓
            521.7422     426.41406    ↑
            588.08203    426.13672    ↓
            588.08203    426.13672    ↑
            743.0469     428.61328    ↓
            743.0469     428.61328    ↑
            427.14844    754.0625     ↓
            427.14844    754.0625     ↑
            429.39844    588.0039     ↓
            429.39844    587.7617     ↑
            426.92188    516.4336     ↓
            426.92188    516.1836     ↑
            425.40234    334.46094    ↓
            425.40234    334.46094    ↑
            430.125      217.41406    ↓
            430.125      217.17578    ↑
            426.8711     88.13281     ↓
            426.8711     88.13281     ↑
            "#,
    )
    .await;

    if get_state::<String>()
        != "_le_s_ct_left_le_ct_center_ri_ct_right_ri_s_ct_bo_s_ct_bottom_bt_ct_tp_ct_top_to_s_ct"
    {
        bail!("Fail");
    }

    clear_state();

    from_main(|| {
        Screen::current().set_size((1600, 1200).into());
    })
    .await;

    inject_touches(
        r#"
            809.1758     943.28906    ↓
            808.9336     943.28906    ↑
            811.6875     779.2383     ↓
            811.4453     779.2383     ↑
            797.7578     649.96484    ↓
            797.7578     649.96484    ↑
            806.0625     503.38672    ↓
            806.0625     503.38672    ↑
            804.52344    368.69922    ↓
            804.52344    368.69922    ↑
            807.1953     244.3789     ↓
            807.1953     244.14453    ↑
            806.5625     104.24219    ↓
            806.5625     104.24219    ↑
            1503.7461    516.08984    ↓
            1503.7461    516.08984    ↑
            1343.5664    516.33203    ↓
            1343.3281    516.33203    ↑
            1083.8633    522.21094    ↓
            1083.8633    522.21094    ↑
            819.1758     518.9883     ↓
            819.1758     518.9883     ↑
            536.1914     521.2617     ↓
            535.9492     521.2617     ↑
            269.59375    520.2383     ↓
            269.35547    520.2383     ↑
            102.12891    523.53906    ↓
            102.12891    523.53906    ↑
            "#,
    )
    .await;

    if get_state::<String>()
        == "_bo_s_ct_bottom_bt_ct_center_tp_ct_top_to_s_ct_ri_s_ct_right_ri_ct_center_le_ct_left_le_s_ct"
    {
        debug!("Layout test: OK");
        Ok(())
    } else {
        bail!("Fail")
    }
}
