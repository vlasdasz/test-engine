use crate::base::{
    keymap::test_keymap, layout::test_layout, modal_test::test_modal, on_tap_add::test_add_on_tap,
    out_bounds_test::test_out_bounds, present_test::test_present, selection::test_selection,
    template::test_template, text_occlusion::test_text_occlusion, touch_order::test_touch_order,
    touch_stack::test_touch_stack, view_order::test_view_order,
};

mod keymap;
mod layout;
mod modal_test;
mod on_tap_add;
mod out_bounds_test;
mod present_test;
mod selection;
mod template;
mod text_occlusion;
mod touch_order;
mod touch_stack;
mod view_order;

pub async fn test_base_ui() -> anyhow::Result<()> {
    test_layout().await?;
    test_out_bounds().await?;
    test_modal().await?;
    test_view_order().await?;
    test_add_on_tap().await?;
    test_touch_order().await?;
    test_template().await?;
    test_present().await?;
    test_touch_stack().await?;
    test_text_occlusion().await?;
    test_selection().await?;
    test_keymap().await?;

    Ok(())
}
