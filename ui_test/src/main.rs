#![allow(incomplete_features)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

use anyhow::Result;
use test_engine::{
    ui::{Container, ViewSetup},
    App,
};

use crate::views::{
    alert::test_alert, button::test_button, collection_view::test_collection_view, drop_down::test_drop_down,
    image_view::test_image_view, int_view::test_int_view, keymap::test_keymap, label::test_label,
    layout::test_layout, modal_test::test_modal, multiline_label::test_multiline,
    out_bounds_test::test_out_bounds, point_view::test_point_view, present_test::test_present,
    render_image_path::test_render_image_path, scroll_view::test_scroll_view, selection::test_selection,
    slider::test_slider, stick::test_stick, switch::test_switch, text_field::test_text_field,
    text_occlusion::test_text_occlusion, touch_stack::test_touch_stack, view_order::test_view_order,
};

mod view_tests;
mod views;

#[tokio::main]
async fn main() -> Result<()> {
    App::start_with_actor(Container::new(), async {
        test_engine::ui::UIManager::set_display_touches(true);

        for _ in 0..2 {
            test().await?;
        }

        _ = skip();

        Ok(())
    })
    .await
}

async fn test() -> Result<()> {
    test_scroll_view().await?;
    test_collection_view().await?;
    test_present().await?;
    test_stick().await?;
    test_point_view().await?;
    test_out_bounds().await?;
    test_modal().await?;
    test_touch_stack().await?;
    test_text_occlusion().await?;
    test_text_field().await?;
    test_slider().await?;
    test_selection().await?;
    test_keymap().await?;
    test_image_view().await?;
    test_int_view().await?;
    test_button().await?;
    test_switch().await?;
    test_layout().await?;
    test_label().await?;
    test_alert().await?;
    test_multiline().await?;
    test_view_order().await?;

    Ok(())
}

async fn skip() -> Result<()> {
    test_drop_down().await?;
    test_render_image_path().await?;

    Ok(())
}
