use gl_image::GlImage;
use gm::Color;
use refs::Weak;
use ui::view;

#[view]
pub struct ImageView {
    pub image:      Weak<GlImage>,
    pub tint_color: Color,
}
