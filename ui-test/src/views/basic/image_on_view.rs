use anyhow::Result;
use log::debug;
use test_engine::{
    App,
    refs::Weak,
    ui::{Color, Container, ImageView, Setup, UI, UIImages, ViewData, ViewSubviews, view},
    ui_test::helpers::check_colors,
};

#[view]
struct ImageOnViewTestView {
    image: Weak<ImageView>,

    #[init]
    container: Container,
}

impl Setup for ImageOnViewTestView {
    fn setup(mut self: Weak<Self>) {
        self.container.set_color(Color::GREEN).place().size(200, 200).center();

        self.image = self.container.add_view();

        self.image.set_image(UIImages::rb()).place().size(100, 100).center();
    }
}

pub async fn test_image_on_view() -> Result<()> {
    debug!("Image on view:");

    UI::init_test_view::<ImageOnViewTestView>().await;

    App::set_window_size((400, 400)).await;

    check_colors(
        r#"
              73  197 -  25  51  76
              80  197 -  25  51  76
             103  200 -   0 255   0
             127  196 -   0 255   0
             147  197 -   0 255   0
             168  196 -   0 255   0
             227  197 -  14  14  14
             250  198 -   0 255   0
             290  199 -   0 255   0
             314  198 -  25  51  76
             325  188 -  25  51  76
             323  167 -  25  51  76
             294  102 -   0 255   0
             271   68 -  25  51  76
             236  100 -   0 255   0
             231  138 -   0 255   0
             225  161 -   0 255   0
             222  220 -  14  14  14
             220  252 -   0 255   0
             216  321 -  25  51  76
             199  329 -  25  51  76
             183  314 -  25  51  76
             175  296 -   0 255   0
             174  273 -   0 255   0
             173  256 -   0 255   0
             166  217 -   0 255   0
             158  194 -   0 255   0
             137  130 -   0 255   0
              99  101 -  25  51  76
              85   82 -  25  51  76
             104  156 -   0 255   0
             180  157 -   0 255   0
             220  172 -   0 255   0
             246  191 -  14  14  14
             290  190 -   0 255   0
             326  200 -  25  51  76
        "#,
    )
    .await?;

    debug!("OK");

    Ok(())
}
