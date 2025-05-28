use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{ImageView, Setup, UI, ViewData, ViewTouch, view},
    ui_test::helpers::check_colors,
};

#[view]
struct ImageViewSVG {
    #[init]
    image_view: ImageView,
}

impl Setup for ImageViewSVG {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();

        self.image_view.place().tl(5).size(400, 400);
        self.image_view.set_image("bin.svg");
    }
}

pub async fn test_image_view_svg() -> Result<()> {
    let _view = UI::init_test_view::<ImageViewSVG>().await;

    check_colors(
        r#"
              63  197 -  89 124 149
             116  203 -   0 123 255
             146  212 -  89 124 149
             170  213 -   0 123 255
             210  217 -  89 124 149
             237  216 -   0 123 255
             273  215 -  89 124 149
             304  215 -   0 123 255
             346  216 -  89 124 149
             227  393 -  89 124 149
             225  361 -   0 123 255
             228  308 -  89 124 149
             234  181 -   0 123 255
             210  128 -  89 124 149
             211  105 -   0 123 255
             211   87 -  89 124 149
             207   57 -   0 123 255
             168   11 -  89 124 149
              39   59 -  89 124 149
              95  104 -   0 123 255
             124   69 -  89 124 149
             296   71 -  89 124 149
             329  106 -   0 123 255
             377  341 -  89 124 149
             292  356 -   0 123 255
             311  378 -  89 124 149
              97  397 -  89 124 149
              77  195 -  89 124 149
        "#,
    )
    .await?;

    Ok(())
}
