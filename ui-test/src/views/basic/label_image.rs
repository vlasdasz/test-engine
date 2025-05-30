use std::any::Any;

use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{HasText, LIGHT_BLUE, Label, Setup, TableData, TableView, UI, ViewData, WHITE, view},
    ui_test::helpers::check_colors,
};

#[view]
struct LabelImage {
    #[init]
    label:      Label,
    table_view: TableView,
}

impl Setup for LabelImage {
    fn setup(mut self: Weak<Self>) {
        self.label.set_text("ßšėčыў").set_text_size(110).set_image("cat.png");
        self.label.place().tl(50).w(400).h(200);

        self.table_view.set_data_source(self);
        self.table_view.place().below(self.label, 20);
        self.table_view.set_color(LIGHT_BLUE);
    }
}

impl TableData for LabelImage {
    fn cell_height(self: Weak<Self>) -> f32 {
        50.0
    }

    fn number_of_cells(self: Weak<Self>) -> usize {
        4
    }

    fn setup_cell(self: Weak<Self>, cell: &mut dyn Any, index: usize) {
        let label = cell.downcast_mut::<Label>().unwrap();

        label.set_text(index);
        label.set_text_size(50);
        label.set_text_color(WHITE);
        label.set_image("cat.png");
    }
}

pub async fn test_label_image() -> Result<()> {
    let mut view = UI::init_test_view::<LabelImage>().await;

    check_colors(
        r#"
              36  154 -  89 124 149
              67  154 - 228 178 181
              74  154 - 130 100 102
              75  154 -   0   0   0
              79  153 -   0   0   0
              88  153 - 234 186 186
             102  153 - 239 210 204
             118  158 -   0   0   0
             138  158 - 233 200 183
             160  155 - 122 102  91
             173  155 -   0   0   0
             223  143 - 168 137 109
             247  143 - 182 151 122
             255  143 -   5   3   2
             257  143 -   5   3   2
             290  143 -  64  55  26
             310  149 -   2   1   1
             322  152 - 179 148 119
             372  153 -   6   4   2
             373  154 -   6   4   2
             393  154 - 200 150 149
             400  154 -   0   0   0
             422  154 -   0   0   0
             442  154 - 201 151 152
             420  110 -  36  25  19
             406  110 -   0   0   0
             217  113 -   0   0   0
             217  113 -   0   0   0
             217  113 -   0   0   0
             565  149 -  89 124 149
             322  174 -   7   5   3
        "#,
    )
    .await?;

    from_main(move || {
        view.label.set_resizing_image("button");
        view.label.set_text_color(WHITE);
    })
    .await;

    check_colors(
        r#"
              42  158 -  89 124 149
              60  156 -   2  19  64
              71  154 -   4  18  61
              80  154 - 255 255 255
              91  154 -   4  18  63
             113  157 - 255 255 255
             127  157 -   4  19  64
             139  156 -   5  18  65
             155  157 -   5  18  65
             215  156 -   4  18  65
             237  156 -   4  18  64
             248  156 -   5  19  65
             270  156 -   4  19  65
             304  156 -   4  18  64
             315  156 - 255 255 255
             349  156 - 255 255 255
             361  155 -   5  19  65
             375  155 -   5  18  64
             381  155 -   5  18  65
             395  155 - 255 255 255
             416  153 - 255 255 255
             425  154 -   5  18  64
             441  154 -   5  18  61
             482  154 -  89 124 149
             426  223 -   3  17  63
             440  238 -  89 124 149
              66   32 -  89 124 149
              66   85 -   3  17  63
             110   90 -   4  17  64
        "#,
    )
    .await?;

    // record_ui_test().await;

    Ok(())
}
