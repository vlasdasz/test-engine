use crate::views::layout::tiling_layout::test_tiling_layout;

mod tiling_layout;

pub async fn test_layout() -> anyhow::Result<()> {
    test_tiling_layout().await?;
    Ok(())
}
