use crate::views::complex::{
    alert::test_alert, buttons_on_table::test_buttons_on_table_view, collection_view::test_collection_view,
    drop_down::test_drop_down, form::test_form_view, number_view::test_number_view,
    point_view::test_point_view, table_view::test_table_view,
};

mod alert;
mod buttons_on_table;
mod collection_view;
mod drop_down;
mod form;

mod number_view;
mod point_view;
mod table_view;

pub async fn test_complex_views() -> anyhow::Result<()> {
    test_number_view().await?;
    test_form_view().await?;
    test_table_view().await?;
    test_collection_view().await?;
    test_drop_down().await?;
    test_buttons_on_table_view().await?;
    test_point_view().await?;
    test_alert().await?;

    Ok(())
}
