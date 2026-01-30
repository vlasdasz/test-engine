use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{BLACK, Container, Setup, TextField, UIDrawer, ViewData, ViewFrame, ViewSubviews, view},
};

#[view]
struct CustomTextField {
    #[init]
    field: TextField,
}

impl Setup for CustomTextField {
    fn setup(mut self: Weak<Self>) {
        self.field.set_text("1.eĘEŠ").set_text_size(400);
        self.field.set_frame((20, 20, 800, 500));
        
        self.field.add_view::<Container>().set_color(BLACK).place().lr(0).center_y().h(10);
    }
}

pub async fn test_custom_text_field() -> Result<()> {
    let view = UIDrawer::init_test_view::<CustomTextField>();
    
    dbg!(view.field.frame().center());
    
    // (145, 270);
    
    // test_engine::ui_test::record_ui_test();

    Ok(())
}
