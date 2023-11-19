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
        Screen::current().set_size((1600, 1000));
    })
    .await;

    clear_state();

    inject_touches(
        r#"
            77           517          b
            110          485          e
            204          478          b
            238          513          e
            526          482          b
            505          517          e
            787          517          b
            815          478          e
            1069         481          b
            1104         516          e
            1391         481          b
            1361         517          e
            1482         518          b
            1514         482          e
            814          915          b
            793          883          e
            818          754          b
            783          793          e
            786          652          b
            816          618          e
            815          514          b
            786          481          e
            789          376          b
            814          346          e
            817          243          b
            784          206          e
            785          80           b
            816          114          e
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
