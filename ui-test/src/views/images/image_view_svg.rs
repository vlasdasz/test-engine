use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{Anchor::Top, BLUE, ImageView, Setup, Tinted, ViewData, ViewTouch, view},
    ui_test::{UITest, helpers::check_colors},
};

#[view]
struct ImageViewSVG {
    #[init]
    bin:      ImageView,
    settings: ImageView,
}

impl Setup for ImageViewSVG {
    fn setup(self: Weak<Self>) {
        self.enable_touch();

        self.bin.place().tl(5).size(400, 400);
        self.bin.set_image("bin.svg");

        self.settings.place().same_x(self.bin).anchor(Top, self.bin, 20).size(150, 150);
        self.settings.set_image("settings.svg");
    }
}

pub async fn test_image_view_svg() -> Result<()> {
    let view = UITest::init::<ImageViewSVG>();

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
    )?;

    check_colors(
        r#"
              16  527 -  89 124 149
              43  520 -   0   0   0
              76  505 -  89 124 149
             117  479 -   0   0   0
             141  477 -  89 124 149
        "#,
    )?;

    from_main(move || {
        view.settings.set_image(Tinted {
            tint: BLUE,
            name: "settings.svg".to_string(),
        });
    });

    check_colors(
        r#"
              20  526 -  89 124 149
              44  523 -   0   0 153
              80  510 -  89 124 149
             128  484 -   0   0 153
             152  479 -  89 124 149
        "#,
    )?;

    Ok(())
}
