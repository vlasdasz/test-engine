use crate::views::window_resize::paths_resize::test_paths_resize;

mod paths_resize;

pub async fn test_window_resize() -> anyhow::Result<()> {
    test_paths_resize().await?;
    Ok(())
}
