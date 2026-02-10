use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{LIGHT_GRAY, Label, Setup, ViewData, view},
    ui_test::{UITest, helpers::check_colors},
};

#[view]
pub struct TextOccclusion {
    #[init]
    label_below: Label,
    label_above: Label,
}

impl Setup for TextOccclusion {
    fn setup(self: Weak<Self>) {
        self.label_below
            .set_text_size(100)
            .set_text("OOOOOOOO")
            .place()
            .size(400, 400)
            .center();

        self.label_above
            .set_text_size(140)
            .set_text("A A A A A")
            .set_color(LIGHT_GRAY)
            .place()
            .right_half();
    }
}

pub async fn test_text_occlusion() -> Result<()> {
    let _view = UITest::start::<TextOccclusion>();

    check_colors(
        r#"
              73  295 -  89 124 149
              81  289 -  89 124 149
             104  287 - 255 255 255
             113  285 - 255 255 255
             114  285 - 255 255 255
             123  285 - 255 255 255
             142  284 - 255 255 255
             148  284 - 233 233 233
             161  281 - 255 255 255
             183  275 - 255 255 255
             185  276 - 255 255 255
             193  274 - 255 255 255
             198  276 - 255 255 255
             211  281 -  14  14  14
             219  282 - 255 255 255
             243  290 - 255 255 255
             282  293 - 255 255 255
             292  288 -  14  14  14
             298  289 - 255 255 255
             307  288 - 231 231 231
             319  286 - 231 231 231
             333  288 - 231 231 231
             350  290 - 231 231 231
             354  291 - 231 231 231
             361  301 -   0   0   0
             366  298 -   0   0   0
             373  299 -   0   0   0
             381  299 -   0   0   0
             397  301 -   0   0   0
             401  302 -   0   0   0
             422  302 - 231 231 231
             455  305 - 231 231 231
             465  302 - 231 231 231
             484  302 - 208 208 208
             492  299 -   0   0   0
             501  298 -   0   0   0
             506  298 -   0   0   0
             517  301 -   0   0   0
             530  310 - 231 231 231
             543  311 -   0   0   0
             548  327 -  11  11  11
             543  331 - 230 230 230
             514  314 - 231 231 231
             335  267 - 231 231 231
             274  174 - 255 255 255
             245  103 - 255 255 255
             228  102 - 255 255 255
             178   69 -  89 124 149
             145   76 -  89 124 149
              71  239 -  89 124 149
             119  416 - 255 255 255
             209  519 -  89 124 149
             314  522 - 231 231 231
             381  405 - 231 231 231
             421  325 -   0   0   0
        "#,
    )?;

    Ok(())
}
