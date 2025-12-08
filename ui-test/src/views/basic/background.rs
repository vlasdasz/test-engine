use anyhow::Result;
use test_engine::{
    Window,
    dispatch::from_main,
    refs::Weak,
    ui::{GRAY_BLUE, GREEN, NoImage, Setup, UI, UIImages, UIManager, view},
    ui_test::check_colors,
};

#[view]
struct Background {}

impl Setup for Background {
    fn setup(self: Weak<Self>) {
        Window::set_clear_color(GREEN);
    }
}

pub async fn test_background() -> Result<()> {
    let _view = UI::init_test_view::<Background>();

    check_colors(
        r#"
             267  205 -   0 255   0
             286  263 -   0 255   0
             380  272 -   0 255   0
        "#,
    )?;

    from_main(|| {
        Window::set_clear_color(GRAY_BLUE);
    });

    check_colors(
        r#"
             236  242 -  89 124 149
             176  334 -  89 124 149
             372  357 -  89 124 149
        "#,
    )?;

    from_main(|| {
        UIManager::root_view().set_image(UIImages::up());
    });

    check_colors(
        r#"
             131  269 -   0 150 230
             275  323 - 255 255 255
             429  246 -   0 150 230
             592   11 -  89 124 149
               8    9 -  89 124 149
        "#,
    )?;

    from_main(|| {
        UIManager::root_view().set_image(NoImage);
    });

    check_colors(
        r#"
             236  242 -  89 124 149
             176  334 -  89 124 149
             372  357 -  89 124 149
        "#,
    )?;

    Ok(())
}
