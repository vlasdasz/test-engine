use crate::views::basic::{
    button::test_button, gradient::test_gradient, inject_touch::test_inject_touch, label::test_label,
    label_image::test_label_image, multiline_label::test_multiline, nine_segment::test_nine_segment,
    scroll_view::test_scroll_view, slider::test_slider, stick::test_stick, switch::test_switch,
    text_field::test_text_field,
};

mod button;
mod gradient;
mod inject_touch;
mod label;
mod label_image;
mod multiline_label;
mod nine_segment;
mod scroll_view;
mod slider;
mod stick;
mod switch;
mod text_field;

pub async fn test_base_views() -> anyhow::Result<()> {
    test_stick().await?;
    test_label_image().await?;
    test_label().await?;
    test_nine_segment().await?;
    test_gradient().await?;
    test_multiline().await?;
    test_button().await?;
    test_inject_touch().await?;
    test_scroll_view().await?;
    test_slider().await?;
    test_switch().await?;
    test_text_field().await?;

    Ok(())
}
