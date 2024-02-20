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
    alert::test_alert, button::test_button, drop_down::test_drop_down, image_view::test_image_view,
    int_view::test_int_view, keymap::test_keymap, label::test_label, layout::test_layout,
    multiline_label::test_multiline, render_image_path::test_render_image_path, switch::test_switch,
    touch_stack::test_touch_stack,
};

mod view_tests;
mod views;

#[tokio::main]
async fn main() -> Result<()> {
    App::start_with_actor(Container::new(), async {
        test_engine::ui::UIManager::set_display_touches(true);

        test_keymap().await?;
        test_image_view().await?;
        test_button().await?;
        test_int_view().await?;
        test_switch().await?;
        test_layout().await?;
        test_label().await?;
        test_alert().await?;
        test_multiline().await?;

        _ = skip();

        Ok(())
    })
    .await
}

async fn skip() -> Result<()> {
    test_drop_down().await?;
    test_touch_stack().await?;
    test_render_image_path().await?;

    Ok(())
}
