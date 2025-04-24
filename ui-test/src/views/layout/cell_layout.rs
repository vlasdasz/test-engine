use test_engine::{
    refs::Weak,
    ui::{Anchor::Right, Button, Label, Setup, UI, UIImages, ViewData, view},
    ui_test::check_colors,
};

#[view]
struct CellLayout {
    #[init]
    delete: Button,
    label:  Label,
}

impl Setup for CellLayout {
    fn setup(mut self: Weak<Self>) {
        self.delete.place().t(100).l(400).size(50, 50);
        self.delete.set_image(UIImages::delete());

        self.label.place().l(50).t(100).h(200).anchor(Right, self.delete, 10);
    }
}

pub async fn test_cell_layout() -> anyhow::Result<()> {
    let _view = UI::init_test_view::<CellLayout>().await;

    check_colors(
        r#"
              31  242 -  89 124 149
              57  238 - 255 255 255
              57  303 -  89 124 149
              87  342 -  89 124 149
             374  315 -  89 124 149
             374  255 - 255 255 255
             437  252 -  89 124 149
             460  157 -  89 124 149
             446  140 - 255 255 255
             421   69 -  89 124 149
             421  133 -  80 197 255
             394  133 -  89 124 149
             354  131 - 255 255 255
             354  131 - 255 255 255
             362   66 -  89 124 149
             405   71 -  89 124 149
             406  157 -  89 124 149
             356  231 - 255 255 255
             229  305 -  89 124 149
              88  348 -  89 124 149
              25  237 -  89 124 149
              64  144 - 255 255 255
              70   76 -  89 124 149
             310   17 -  89 124 149
             387   73 -  89 124 149
             425  133 -  30  63 107
             425  206 -  89 124 149
             378  321 -  89 124 149
             175  359 -  89 124 149
             481  126 -  89 124 149
        "#,
    )
    .await?;

    Ok(())
}
