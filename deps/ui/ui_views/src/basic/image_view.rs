use gl_image::Image;
use gm::Color;
use rtools::data_manager::Handle;
use ui::view;

#[view]
pub struct ImageView {
    pub image:      Handle<Image>,
    pub tint_color: Color,
}
