use std::{any::Any, ops::Deref};

use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::{Own, Weak},
    ui::{
        CollectionData, CollectionLayout, CollectionView, HasText, Label, Setup, Size, TouchStack, UI, View,
        ViewData, ViewFrame, view,
    },
    ui_test::{add_action, helpers::check_colors},
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

    fn setup_cell_for_index(&self, cell: &mut dyn Any, index: usize) {
        let cell = cell.downcast_mut::<Label>().unwrap();
        cell.set_text(&self.records[index]);
    }

    fn size_for_index(&self, _index: usize) -> Size {
        (105, 50).into()
    }

    fn make_cell(&self) -> Own<dyn View> {
        Label::new()
    }
}

impl Setup for CollectionTestView {
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
              57  408 -  89 124 149
              92  349 -  89 124 149
             135  306 -  89 124 149
             163  274 -  89 124 149
             202  240 -  89 124 149
             242  210 -  89 124 149
             364   95 -  89 124 149
             370   90 -  89 124 149
             503   27 -  89 124 149
             190   43 -  89 124 149
             110   43 -  89 124 149
             199  285 -  89 124 149
             399  531 -  89 124 149
             493  523 -  89 124 149
             295  266 -  89 124 149
             282  263 -  89 124 149
        "#,
    )
    .await?;

    from_main(move || {
        view.records.push("Hello".to_string());
        view.table.reload_data();
    });

    check_colors(
        r#"
             200   68 -  89 124 149
             205   64 -  89 124 149
             210   61 -  89 124 149
             226   55 -  89 124 149
             243   44 - 255 255 255
             243   44 - 255 255 255
             268   39 - 255 255 255
             282   37 - 255 255 255
             295   37 - 255 255 255
             314   20 -   0   0   0
             314   20 -   0   0   0
             317   20 - 255 255 255
             346   20 - 255 255 255
             346   20 - 255 255 255
             332   20 -   2   2   2
             330   20 - 242 242 242
             284   20 - 222 222 222
             251   18 - 255 255 255
             436   19 - 255 255 255
             436   19 - 255 255 255
             436  186 -  89 124 149
        "#,
    )
    .await?;

    from_main(move || {
        for i in 0..8 {
            view.records.push(format!("Hello {i}"));
        }

        view.table.reload_data();
    });

    check_colors(
        r#"
             293  482 -  89 124 149
             293  461 -  89 124 149
             293  435 - 255 255 255
             293  424 - 135 135 135
             293  391 - 255 255 255
             293  378 - 135 135 135
             293  365 - 135 135 135
             298  307 - 255 255 255
             349  213 - 255 255 255
             352  206 - 255 255 255
             342  180 - 254 254 254
             304  163 - 255 255 255
             295  157 - 255 255 255
             295  103 - 255 255 255
             295   92 - 255 255 255
             261    8 - 255 255 255
             188   33 - 255 255 255
             105  136 - 255 255 255
              98  164 - 255 255 255
              93  238 - 255 255 255
              92  277 - 255 255 255
              82  378 - 255 255 255
              19  452 -  89 124 149
              30  476 -  89 124 149
             454  477 -  89 124 149
             567  471 -  89 124 149
             533  377 - 255 255 255
             533  368 - 255 255 255
             533  331 - 255 255 255
             533  280 - 255 255 255
             531  255 - 255 255 255
        "#,
    )
    .await?;

    from_main(move || {
        view.table.place().clear().left_half();
        view.collection.place().right_half();
        view.collection.layout = CollectionLayout::Cards;
        view.collection.reload_data();
    });

    check_colors(
        r#"
              63  468 -  89 124 149
              74  460 -  89 124 149
              88  442 - 255 255 255
             108  405 - 255 255 255
             113  393 - 255 255 255
             116  385 - 255 255 255
             148  351 - 255 255 255
             176  321 - 255 255 255
             192  307 - 255 255 255
             235  277 - 255 255 255
             267  230 - 255 255 255
             268  221 - 255 255 255
             275  188 - 255 255 255
             291  161 - 255 255 255
             337  144 - 255 255 255
             337  144 - 255 255 255
             340  144 - 255 255 255
             401  143 - 255 255 255
             412  142 - 255 255 255
             412  142 - 255 255 255
             449  129 - 255 255 255
             453  123 - 181 181 181
             507  111 - 255 255 255
             507  111 - 255 255 255
             547   94 -  89 124 149
             547   94 -  89 124 149
             511  145 -  89 124 149
             489  164 - 255 255 255
             472  170 - 255 255 255
             457  186 - 255 255 255
             450  198 - 255 255 255
             445  198 - 255 255 255
             421  214 -  89 124 149
             407  227 -  89 124 149
             405  239 -  89 124 149
             396  254 -  89 124 149
             373  270 -  89 124 149
             369  277 -  89 124 149
             321  363 -  89 124 149
             271  435 - 255 255 255
             271  435 - 255 255 255
             221  457 -  89 124 149
             216  459 -  89 124 149
             177  286 - 255 255 255
             177  264 - 255 255 255
             171  100 - 255 255 255
             171  100 - 255 255 255
             169   91 - 255 255 255
             443  148 - 255 255 255
             471  148 - 255 255 255
             471  148 - 255 255 255
             552  152 -  89 124 149
        "#,
    )
    .await?;

    Ok(())
}
