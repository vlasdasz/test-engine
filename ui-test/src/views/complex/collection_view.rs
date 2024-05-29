use std::{any::Any, ops::Deref};

use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    refs::{Own, Weak},
    ui::{
        view, CollectionData, CollectionLayout, CollectionView, Label, Size, TouchStack, View, ViewData,
        ViewFrame, ViewSetup, UI,
    },
    ui_test::helpers::{add_action, check_colors},
};

#[view]
struct CollectionTestView {
    records: Vec<String>,

    #[init]
    table:      CollectionView,
    collection: CollectionView,
}

impl CollectionData for CollectionTestView {
    fn number_of_cells(&self) -> usize {
        self.records.len()
    }

    fn make_cell(&self) -> Own<dyn View> {
        Label::new()
    }

    fn setup_cell_for_index(&self, cell: &mut dyn Any, index: usize) {
        let cell = cell.downcast_mut::<Label>().unwrap();
        cell.set_text(&self.records[index]);
    }

    fn size_for_index(&self, _index: usize) -> Size {
        (105, 50).into()
    }
}

impl ViewSetup for CollectionTestView {
    fn setup(self: Weak<Self>) {
        self.table.place().back();
        self.table.set_data_source(self.deref());
        self.collection.set_data_source(self.deref());

        add_action(move || {
            dbg!(TouchStack::dump());
            dbg!(self.collection.frame());
        });
    }
}

pub async fn test_collection_view() -> Result<()> {
    let mut view = UI::init_test_view::<CollectionTestView>().await;

    check_colors(
        r#"
              33   34 -  25  51  76
             117  145 -  25  51  76
             229  255 -  25  51  76
             329  354 -  25  51  76
             449  467 -  25  51  76
             568  569 -  25  51  76
        "#,
    )
    .await?;

    from_main(move || {
        view.records.push("Hello".to_string());
        view.table.reload_data();
    })
    .await;

    check_colors(
        r#"
             238   61 -  25  51  76
             249   26 - 255 255 255
             258   25 - 255 255 255
             277   25 - 255 255 255
             344   25 - 255 255 255
             300   23 -  19  19  19
             353   41 - 255 255 255
             355   72 -  25  51  76
        "#,
    )
    .await?;

    from_main(move || {
        for i in 0..8 {
            view.records.push(format!("Hello {i}"));
        }

        view.table.reload_data();
    })
    .await;

    check_colors(
        r#"
             296  474 -  25  51  76
             298  382 - 255 255 255
             299  327 - 255 255 255
             298  249 - 255 255 255
             258   43 - 255 255 255
             258   43 - 255 255 255
             258  126 - 255 255 255
             258  126 - 255 255 255
             258  178 - 255 255 255
             266  202 - 255 255 255
             395  333 - 255 255 255
             326  300 - 255 255 255
             330  350 - 255 255 255
             353  381 - 255 255 255
             342  413 -   0   0   0
             338  456 -  25  51  76
             338  557 -  25  51  76
        "#,
    )
    .await?;

    from_main(move || {
        view.table.place().clear().left_half();
        view.collection.place().right_half();
        view.collection.layout = CollectionLayout::Cards;
        view.collection.reload_data();
    })
    .await;

    check_colors(
        r#"
              14  467 -  25  51  76
              37  449 - 255 255 255
              58  431 - 255 255 255
             112  398 - 255 255 255
             122  366 - 255 255 255
             166  298 - 255 255 255
             180  284 - 255 255 255
             180  266 - 255 255 255
             190  230 - 255 255 255
             320  232 -  74  74  74
             317  275 -  25  51  76
             365  234 - 215 215 215
             425  231 -  25  51  76
             440  221 -  25  51  76
             513  143 -  25  51  76
             540   81 -  25  51  76
             526   22 -  25  51  76
             506   21 - 255 255 255
             474   21 - 255 255 255
             443   37 - 255 255 255
             427   87 - 255 255 255
             415  129 - 255 255 255
             397  119 -   0   0   0
             321  119 -   0   0   0
             320  118 -  74  74  74
             320  177 -  74  74  74
             320  252 -  25  51  76
        "#,
    )
    .await?;

    debug!("Collection test: OK");

    Ok(())
}
