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
                append_state(&format!("|{}", button.text()));
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
        "|le_s_ct|left|le_ct|center|ri_ct|right|ri_s_ct|bo_s_ct|bottom|bt_ct|tp_ct|top|to_s_ct",
    )?;

    from_main(|| {
        Screen::current().set_size((1600, 1200));
    })
    .await;

    clear_state();

    inject_touches(
        r#"
            100.27734    514.8164     b
            100.27734    514.8164     e
            232.60938    511.27344    b
            232.60938    511.27344    e
            525.23047    508.5703     b
            525.23047    508.5703     e
            811.8242     511.26172    b
            811.8242     511.26172    e
            1071.207     509.9375     b
            1078.8047    521.59766    e
            1343.625     493.53516    b
            1352.3945    521.59766    e
            1508.2813    529.85156    b
            1482.7148    502.85156    e
            788.77734    946.5508     b
            803.9961     929.6328     e
            841.47656    825.5156     b
            766.6289     744.2344     e
            786.58984    668.5117     b
            809.4375     645.2344     e
            844.53906    549.71484    b
            769.9414     492.5078     e
            785.27344    401.30078    b
            815.8711     368.79297    e
            843.2031     284.5078     b
            758.78516    206.8086     e
            818.83984    85.80859     b
            791.7539     110.125      e
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
