use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, SubView, TextField, ViewData, ViewSetup},
    App,
};

use crate::view_tests::record_touches;

#[view]
struct TextFieldTestView {
    field: SubView<TextField>,
}

impl ViewSetup for TextFieldTestView {
    fn setup(self: Weak<Self>) {
        self.field.place().center().size(200, 50);
    }
}

pub async fn test_text_field() -> Result<()> {
    App::set_test_view::<TextFieldTestView>(600, 600).await;

    record_touches().await;

    debug!("Text field test: OK");

    Ok(())
}
