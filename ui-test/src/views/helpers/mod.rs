use crate::views::helpers::{
    highlight::test_highlight, keyboard::test_keyboard_view, position_view::test_position_view,
};

mod highlight;
mod keyboard;
mod position_view;

pub async fn test_helper_views() -> anyhow::Result<()> {
    test_position_view().await?;
    test_keyboard_view().await?;
    test_highlight().await?;
    Ok(())
}
