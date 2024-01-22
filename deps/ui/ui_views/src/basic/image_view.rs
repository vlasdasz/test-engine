use gl_image::Image;
use gm::Color;
use refs::Weak;
use ui::view;

#[view]
pub struct ImageView {
    pub image:      Weak<Image>,
    pub tint_color: Color,
}
