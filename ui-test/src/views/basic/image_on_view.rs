use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{Color, Container, ImageView, Setup, UI, UIImages, ViewData, ViewSubviews, view},
    ui_test::helpers::check_colors,
};

#[view]
struct ImageOnView {
    image: Weak<ImageView>,

    #[init]
    container: Container,
}

impl Setup for ImageOnView {
    fn setup(mut self: Weak<Self>) {
        self.container.set_color(Color::GREEN).place().size(200, 200).tl(100);

        self.image = self.container.add_view();

        self.image.set_image(UIImages::rb()).place().size(100, 100).center();
    }
}

pub async fn test_image_on_view() -> Result<()> {
    UI::init_test_view::<ImageOnView>().await;

    check_colors(
        r#"
              61  209 -  89 124 149
             113  201 -   0 255   0
             128  200 -   0 255   0
             151  197 -   0 255   0
             203  203 -  68  68  68
             231  216 -  68  68  68
             264  218 -   0 255   0
             351  207 -  89 124 149
             353  204 -  89 124 149
             323  175 -  89 124 149
             259  145 -   0 255   0
             219  149 -   0 255   0
             202  203 -  68  68  68
             259  115 -   0 255   0
             202   58 -  89 124 149
             160  117 -   0 255   0
             177  176 -   0 255   0
             223  216 -  68  68  68
             208  262 -   0 255   0
             178  363 -  89 124 149
             108  306 -  89 124 149
             164  228 -   0 255   0
             161  175 -   0 255   0
              63  162 -  89 124 149
             122  177 -   0 255   0
             209  219 -  68  68  68
             269  232 -   0 255   0
             348  204 -  89 124 149
        "#,
    )
    .await?;

    Ok(())
}
