use crate::views::containers::movable_view::test_movable_view;

mod movable_view;

pub async fn test_containers() -> anyhow::Result<()> {
    test_movable_view().await?;
    Ok(())
}
