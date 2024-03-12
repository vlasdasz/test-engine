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

use crate::{
    base::{
        keymap::test_keymap, layout::test_layout, modal_test::test_modal, on_tap_add::test_add_on_tap,
        out_bounds_test::test_out_bounds, present_test::test_present, selection::test_selection,
        template::test_template, text_occlusion::test_text_occlusion, touch_order::test_touch_order,
        touch_stack::test_touch_stack, view_order::test_view_order,
    },
    views::{
        alert::test_alert,
        basic::{
            button::test_button, image_view::test_image_view, label::test_label,
            multiline_label::test_multiline, scroll_view::test_scroll_view, slider::test_slider,
            stick::test_stick, switch::test_switch, text_field::test_text_field,
        },
        collection_view::test_collection_view,
        complex::buttons_on_table::test_buttons_on_table_view,
        drop_down::test_drop_down,
        int_view::test_int_view,
        point_view::test_point_view,
        render_image_path::test_render_image_path,
    },
};

mod base;
mod views;

#[tokio::main]
async fn main() -> Result<()> {
    App::start_with_actor(Container::new(), async {
        test_engine::ui::UIManager::set_display_touches(true);

        for _ in 0..20 {
            test().await?;
        }

        _ = skip();

        Ok(())
    })
    .await
}

async fn test() -> Result<()> {
    test_scroll_view().await?;
    test_int_view().await?;
    test_collection_view().await?;
    test_add_on_tap().await?;
    test_buttons_on_table_view().await?;
    test_touch_order().await?;
    test_template().await?;
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
