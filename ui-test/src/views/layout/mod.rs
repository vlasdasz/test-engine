use center_field::test_center_field;
use tiling_layout::test_tiling_layout;

use crate::views::layout::{
    cell_layout::test_cell_layout, min_width::test_min_width, near_layout::test_near_layout,
    relative_layout::test_relative_layout,
};

mod cell_layout;
mod center_field;
mod min_width;
mod near_layout;
mod relative_layout;
mod tiling_layout;

pub async fn test_layout() -> anyhow::Result<()> {
    test_near_layout().await?;
    test_relative_layout().await?;
    test_cell_layout().await?;
    test_min_width().await?;
    test_center_field().await?;
    test_tiling_layout().await?;
    Ok(())
}
