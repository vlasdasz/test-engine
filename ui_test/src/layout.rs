#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(process_exitcode_internals)]

use std::process::ExitCode;

use test_engine::{
    gm::{flat::IntSize, Color},
    rtools::Apply,
};
use ui::{layout::Anchor, refs::Weak, view, Container, SubView, ViewData, ViewSetup, ViewSubviews, ViewTest};
use ui_views::Button;

use crate::view_tests::{state::set_state, test_combinations};

mod view_tests;

#[view]
struct LayoutTestView {
    center: SubView<Button>,

    top:    SubView<Button>,
    bottom: SubView<Button>,
    left:   SubView<Button>,
    right:  SubView<Button>,

    top_center:    SubView<Button>,
    bottom_center: SubView<Button>,
    left_center:   SubView<Button>,
    right_center:  SubView<Button>,

    top_s_center:    SubView<Button>,
    bottom_s_center: SubView<Button>,
    left_s_center:   SubView<Button>,
    right_s_center:  SubView<Button>,
}

impl ViewSetup for LayoutTestView {
    fn setup(mut self: Weak<Self>) {
        for view in self.subviews_mut() {
            view.place.size(50, 50);
        }

        [self.center, self.top, self.bottom, self.left, self.right].apply(|view| {
            view.place.size(100, 100);
        });

        self.center.place.center();

        self.top.set_text("top").set_color(Color::ORANGE).place.center_hor().t(200);
        self.bottom.set_text("bot").set_color(Color::GREEN).place.center_hor().b(200);
        self.left.set_text("left").place.center_ver().l(200);
        self.right.set_text("right").place.center_ver().r(200);

        self.top_center.clone().set_text("tp ct").place.between(self.top, self.center);
        self.bottom_center
            .clone()
            .set_text("bt ct")
            .place
            .between(self.bottom, self.center);
        self.left_center.clone().set_text("le ct").place.between(self.left, self.center);
        self.right_center
            .clone()
            .set_text("ri ct")
            .place
            .between(self.right, self.center);

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

fn main() -> ExitCode {
    test_engine::ViewApp::<LayoutTestView>::start_with_actor(async {
        return;
        return crate::view_tests::record_touches().await;
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
