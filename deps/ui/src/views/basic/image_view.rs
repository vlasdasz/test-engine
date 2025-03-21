use refs::Weak;
use ui_proc::view;
use window::image::{Image, ToImage};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate as ui;
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
