use refs::Weak;
use ui::view;
use wgpu_wrapper::image::{Image, ToImage};

mod test_engine {
    pub(crate) use refs;
    pub(crate) use ui;
}

#[view]
pub struct ImageView {
    image: Weak<Image>,
}

impl ImageView {
    pub fn image(&self) -> Weak<Image> {
        self.image
    }

    pub fn set_image(&mut self, image: impl ToImage) -> &mut Self {
        self.image = image.to_image();
        self
    }
}
