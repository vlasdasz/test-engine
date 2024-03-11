use anyhow::Result;
use log::debug;
use test_engine::{
    ui::{view, CollectionView, Sub},
    ui_test::record_ui_test,
    App,
};

#[view]
struct ButtonsOnTableView {
    table: Sub<CollectionView>,
}

pub async fn test_buttons_on_table_view() -> Result<()> {
    App::init_test_view::<ButtonsOnTableView>().await;

    record_ui_test().await;

    debug!("Test buttons on table view: OK");

    Ok(())
}
