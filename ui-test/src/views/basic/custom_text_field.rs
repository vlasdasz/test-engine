use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{Setup, TURQUOISE, TextField, UIDrawer, ViewData, ViewFrame, YELLOW, view},
    ui_test::{check_colors, inject_touches},
};

#[view]
struct CustomTextField {
    #[init]
    field: TextField,
}

impl Setup for CustomTextField {
    fn setup(mut self: Weak<Self>) {
        self.field
            .set_text("1.eĘEŠ")
            .set_color(YELLOW)
            .set_border_color(TURQUOISE)
            .set_text_size(50)
            .set_corner_radius(28)
            .set_border_width(10);
        self.field.set_frame((50, 50, 200, 100));
    }
}

pub async fn test_custom_text_field() -> Result<()> {
    let _view = UIDrawer::init_test_view::<CustomTextField>();

    check_colors(
        r#"
                  56  146 -  89 124 149
                  61  140 -   0 255 255
                  73  128 - 255 255   0
                 155  123 -   0   0   0
                 195  122 - 255 255   0
                 215   82 - 255 255   0
                 220   75 - 255 255   0
                 232   69 - 255 255   0
                 236   63 -   0 255 255
                 244   58 -  89 124 149
                 258   47 -  89 124 149
            "#,
    )?;

    inject_touches(
        "
          193  123  b
          193  123  e
      ",
    );

    check_colors(
        r#"
                  57  150 -  89 124 149
                  64  141 -   0 255 255
                  75  134 - 188 188 188
                 124  124 - 188 188 188
                 227   68 - 188 188 188
                 235   64 -   0 255 255
                 240   61 -   0 255 255
                 247   57 -  89 124 149
                  96   70 - 188 188 188
            "#,
    )?;

    inject_touches(
        "
        43   192  b
        43   192  e
    ",
    );

    check_colors(
        r#"
                  56  146 -  89 124 149
                  61  140 -   0 255 255
                  73  128 - 255 255   0
                 155  123 -   0   0   0
                 195  122 - 255 255   0
                 215   82 - 255 255   0
                 220   75 - 255 255   0
                 232   69 - 255 255   0
                 236   63 -   0 255 255
                 244   58 -  89 124 149
                 258   47 -  89 124 149
            "#,
    )?;

    // test_engine::ui_test::record_ui_test();

    Ok(())
}
