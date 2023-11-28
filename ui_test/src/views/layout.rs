use anyhow::Result;
use log::debug;
use test_engine::{from_main, gm::Color, Screen};
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
    Screen::set_test_view::<LayoutTestView>(680, 680).await;

    inject_touches(
        r#"
            87           325          b
            109          346          e
            236          326          b
            215          351          e
            269          353          b
            292          330          e
            322          329          b
            350          352          e
            382          353          b
            407          329          e
            443          330          b
            466          356          e
            598          327          b
            569          354          e
            355          600          b
            326          566          e
            356          441          b
            323          466          e
            324          381          b
            351          406          e
            354          322          b
            322          356          e
            326          264          b
            353          294          e
            356          210          b
            328          234          e
            328          81           b
            355          113          e
            "#,
    )
    .await;

    assert_eq(
        get_state::<String>(),
        "|le_s_ct|left|le_ct|center|ri_ct|right|ri_s_ct|bo_s_ct|bottom|bt_ct|center|tp_ct|top|to_s_ct",
    )?;

    from_main(|| {
        Screen::current().set_size((1600, 960));
    })
    .await;

    clear_state();

    inject_touches(
        r#"
            80.765625    463.9336     b
            116.390625   501.71484    e
            245.92969    465.1289     b
            206.8086     503.86328    e
            494.48047    468.07422    b
            532.97266    501.52344    e
            818.27344    466.32422    b
            780.41797    499.94922    e
            1069.6953    464.1875     b
            1105.0781    499.1836     e
            1394.0781    462.875      b
            1360.5313    498.2422     e
            1484.3906    466.21875    b
            1520.1953    496.25       e
            819.5078     879.72266    b
            784.22656    844.64844    e
            818.375      718.08594    b
            780.98047    749.77344    e
            782.1992     592.0078     b
            821.22266    625.4375     e
            818.4883     498.9961     b
            781.6328     464.1953     e
            782.40234    337.95313    b
            815.3711     370.3672     e
            816.2422     242.1836     b
            781.10547    207.50781    e
            820.375      82.62109     b
            785.97656    116.52734    e
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
