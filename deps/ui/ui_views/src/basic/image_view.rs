use gl_image::Image;
use rtools::data_manager::Handle;
use ui::view;

#[view]
pub struct ImageView {
    pub image: Handle<Image>,
}
