use center_field::test_center_field;
use tiling_layout::test_tiling_layout;

use crate::views::layout::min_width::test_min_width;

mod center_field;
mod min_width;
mod tiling_layout;

pub async fn test_layout() -> anyhow::Result<()> {
    test_min_width().await?;
    test_center_field().await?;
    test_tiling_layout().await?;
    Ok(())
}
