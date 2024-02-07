#![allow(incomplete_features)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

use anyhow::Result;
use old_engine::ui_layer::UILayer;
use ui::Container;

use crate::views::{
    alert::test_alert, button::test_button, drop_down::test_drop_down, image_view::test_image_view,
    int_view::test_int_view, label::test_label, layout::test_layout, multiline_label::test_multiline,
    render_image_path::test_render_image_path, switch::test_switch, touch_stack::test_touch_stack,
};

mod ui_test_legacy;
mod view_tests;
mod views;

fn main() -> Result<()> {
    old_engine::ViewApp::<Container>::start_with_actor(async {
        UILayer::display_touches();

        test_render_image_path().await?;
        test_image_view().await?;
        test_label().await?;
        test_touch_stack().await?;
        test_button().await?;
        test_switch().await?;
        test_layout().await?;
        test_int_view().await?;
        test_drop_down().await?;
        test_alert().await?;
        test_multiline().await?;

        Ok(())
    })
}
