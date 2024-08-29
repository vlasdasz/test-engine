use center_field::test_center_field;
use tiling_layout::test_tiling_layout;

mod center_field;
mod tiling_layout;

pub async fn test_layout() -> anyhow::Result<()> {
    test_center_field().await?;
    test_tiling_layout().await?;
    Ok(())
}
