use crate::base::{
    async_calls::test_async_calls, colors::test_colors, corner_radius::test_corner_radius,
    dispatch::test_dispatch, global_styles::test_global_styles, keymap::test_keymap, layout::test_layout,
    modal_test::test_modal, on_tap_add::test_add_on_tap, out_bounds_test::test_out_bounds,
    present::test_navigation_view, selection::test_selection, styles::test_styles, template::test_template,
    text_occlusion::test_text_occlusion, touch_order::test_touch_order, touch_stack::test_touch_stack,
    transition::test_transition, transparency::test_transparency, view_order::test_view_order,
};

mod async_calls;
mod colors;
mod corner_radius;
mod dispatch;
mod global_styles;
mod keymap;
mod layout;
mod modal_test;
mod on_tap_add;
mod out_bounds_test;
mod present;
mod selection;
mod styles;
mod template;
mod text_occlusion;
mod touch_order;
mod touch_stack;
mod transition;
mod transparency;
mod view_order;

pub async fn test_base_ui() -> anyhow::Result<()> {
    test_view_order().await?;
    test_text_occlusion().await?;
    test_async_calls().await?;
    test_dispatch().await?;
    test_out_bounds().await?;
    test_transition().await?;
    test_global_styles().await?;
    test_styles().await?;
    test_colors().await?;
    test_corner_radius().await?;
    test_transparency().await?;
    test_modal().await?;
    test_touch_order().await?;
    test_template().await?;
    test_navigation_view().await?;
    test_touch_stack().await?;
    test_selection().await?;
    test_keymap().await?;
    test_add_on_tap().await?;
    test_layout().await?;

    Ok(())
}
