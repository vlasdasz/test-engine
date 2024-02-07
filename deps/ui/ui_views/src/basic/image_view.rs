use gm::Color;
use refs::Weak;
use ui::view;
use wgpu_wrapper::image::Image;

#[view]
pub struct ImageView {
    pub image:      Weak<Image>,
    pub tint_color: Color,
}
