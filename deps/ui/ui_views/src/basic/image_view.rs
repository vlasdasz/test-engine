use gl_image::GlImage;
use gm::Color;
use refs::Weak;
use ui::view;
use wgpu_wrapper::image::Image;

#[view]
pub struct ImageView {
    pub image:      Weak<Image>,
    pub gl_image:   Weak<GlImage>,
    pub tint_color: Color,
}
