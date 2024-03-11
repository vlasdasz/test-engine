use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{view, Color, Label, SubView, ViewData, ViewSetup},
    ui_test::helpers::check_colors,
    App,
};

#[view]
pub struct TextOccclusionTestView {
    label_below: SubView<Label>,
    label_above: SubView<Label>,
}

impl ViewSetup for TextOccclusionTestView {
    fn setup(mut self: Weak<Self>) {
        self.label_below
            .set_text_size(100)
            .set_text("OOOOOOOO")
            .place()
            .size(400, 400)
            .center();

        self.label_above
            .set_text_size(140)
            .set_text("A A A A A")
            .set_color(Color::LIGHT_GRAY)
            .place()
            .right_half();
    }
}

pub async fn test_text_occlusion() -> Result<()> {
    App::init_test_view::<TextOccclusionTestView>(600, 600).await;

    check_colors(
        r#"
              74  285 -  25  51  76
             114  284 - 255 255 255
             127  286 - 255 255 255
             145  287 - 255 255 255
             156  287 -   1   1   1
             171  287 - 255 255 255
             182  286 - 255 255 255
             196  285 - 255 255 255
             196  285 - 255 255 255
             226  288 -  81  81  81
             230  288 -   1   1   1
             238  287 - 255 255 255
             257  285 - 255 255 255
             264  283 - 255 255 255
             273  280 - 255 255 255
             289  277 -   1   1   1
             304  277 - 203 203 203
             310  277 - 203 203 203
             323  278 - 203 203 203
             331  278 - 203 203 203
             338  277 - 203 203 203
             355  277 - 203 203 203
             361  277 - 186 186 186
             382  277 - 203 203 203
             394  277 -   0   0   0
             414  277 - 203 203 203
             433  267 - 203 203 203
             443  258 - 203 203 203
             434  247 - 203 203 203
             384  245 -   0   0   0
             362  247 - 203 203 203
             343  250 - 203 203 203
             313  249 - 203 203 203
             286  254 - 255 255 255
             265  290 - 255 255 255
             280  309 - 255 255 255
             330  315 - 203 203 203
             364  311 - 203 203 203
             393  316 - 203 203 203
             421  319 -   0   0   0
             484  325 -   0   0   0
             529  328 - 203 203 203
             570  321 - 203 203 203
             528  296 - 132 132 132
             449  287 - 203 203 203
             390  279 - 203 203 203
             353  272 - 203 203 203
             300  258 - 203 203 203
             250  251 - 255 255 255
             225  252 - 255 255 255
             243  292 - 255 255 255
             270  307 - 255 255 255
             325  315 - 203 203 203
             366  322 - 203 203 203
             405  284 -   0   0   0
             383  233 - 203 203 203
    "#,
    )
    .await?;

    debug!("Text occlusion: OK");

    Ok(())
}
