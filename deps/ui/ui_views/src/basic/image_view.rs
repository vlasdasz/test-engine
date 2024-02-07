use gm::Color;
use refs::Weak;
use ui::view;
use wgpu_wrapper::image::Image;
mod test_engine {
    pub(crate) use refs;
    pub(crate) use ui;
}

#[view]
pub struct ImageView {
    pub image:      Weak<Image>,
    pub tint_color: Color,
}
