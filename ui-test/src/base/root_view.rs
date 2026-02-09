use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    ui::{CLEAR, GREEN, NoImage, UIManager, view},
    ui_test::{UITest, check_colors},
};

#[view]
pub struct RootViewTest {}

pub async fn test_root_view() -> Result<()> {
    UITest::init::<RootViewTest>();

    check_colors(
        r#"
              69   80 -  89 124 149
              49   77 -  89 124 149
              52  157 -  89 124 149
              97  180 -  89 124 149
             169  234 -  89 124 149
             123  320 -  89 124 149
             134  232 -  89 124 149
             285  187 -  89 124 149
             372  123 -  89 124 149
             208   67 -  89 124 149
        "#,
    )?;

    from_main(|| {
        UIManager::root_view().set_color(GREEN);
    });

    check_colors(
        r#"
              89  108 -   0 255   0
             122  197 -   0 255   0
             347  241 -   0 255   0
             374  361 -   0 255   0
             181  427 -   0 255   0
             205  225 -   0 255   0
        "#,
    )?;

    from_main(|| {
        UIManager::root_view().set_color(CLEAR);
    });

    check_colors(
        r#"
              61  153 -  89 124 149
             165  308 -  89 124 149
             369  188 -  89 124 149
             197  180 -  89 124 149
        "#,
    )?;

    from_main(|| {
        UIManager::root_view().set_image("cat.png");
    });

    check_colors(
        r#"
             121  164 - 229 187 188
             145  319 - 222 186 164
             299  300 - 146 118  94
             324  160 - 210 186 162
             146  144 - 226 187 192
             277  288 - 179 149 115
             315  330 - 185 151 116
        "#,
    )?;

    from_main(|| {
        UIManager::root_view().set_image(NoImage);
    });

    check_colors(
        r#"
             131  181 -  89 124 149
             183  326 -  89 124 149
             380  229 -  89 124 149
             181  152 -  89 124 149
        "#,
    )?;

    Ok(())
}
