use crate::views::complex::{
    alert::test_alert, buttons_on_table::test_buttons_on_table_view, collection_view::test_collection_view,
    drop_down::test_drop_down, number_view::test_number_view, number_view_design::test_number_view_design,
    point_view::test_point_view, table_view::test_table_view, table_view_resize::test_table_view_resize,
};

mod alert;
mod buttons_on_table;
mod collection_view;
mod drop_down;

mod number_view;
mod number_view_design;
mod point_view;
mod table_view;
mod table_view_resize;

pub async fn test_complex_views() -> anyhow::Result<()> {
    test_table_view_resize().await?;
    test_drop_down().await?;
    test_number_view_design().await?;
    test_number_view().await?;
    test_alert().await?;
    test_table_view().await?;
    test_collection_view().await?;
    test_buttons_on_table_view().await?;
    test_point_view().await?;

    Ok(())
}
