use crate::views::basic::{
    button::test_button, image_view::test_image_view, inject_touch::test_inject_touch, label::test_label,
    multiline_label::test_multiline, scroll_view::test_scroll_view, slider::test_slider, stick::test_stick,
    switch::test_switch, text_field::test_text_field,
};

mod button;
mod image_view;
mod inject_touch;
mod label;
mod multiline_label;
mod scroll_view;
mod slider;
mod stick;
mod switch;
mod text_field;

pub async fn test_base_views() -> anyhow::Result<()> {
    test_button().await?;
    test_inject_touch().await?;
    test_label().await?;
    test_scroll_view().await?;
    test_slider().await?;
    test_stick().await?;
    test_text_field().await?;
    test_image_view().await?;
    test_switch().await?;
    test_multiline().await?;

    Ok(())
}
