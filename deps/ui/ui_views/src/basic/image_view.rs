use gl_image::Image;
use gm::Color;
use manage::handle::Handle;
use ui::view;

#[view]
pub struct ImageView {
    pub image:      Handle<Image>,
    pub tint_color: Color,
}
