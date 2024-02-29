use gm::flat::Rect;
use refs::Weak;
use ui::{view, ViewFrame};
use wgpu_wrapper::{
    cast_slice,
    image::{Image, ToImage},
    image_vertices_with_shrink, Buffer, BufferInitDescriptor, BufferUsages, DeviceExt, WGPUApp,
};

mod test_engine {
    pub(crate) use refs;
    pub(crate) use ui;
}

#[view]
pub struct ImageView {
    image:   Weak<Image>,
    cropped: Option<Buffer>,
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

impl ImageView {
    pub fn cropped(&self) -> Option<&Buffer> {
        self.cropped.as_ref()
    }

    pub fn check_cropped(mut self: Weak<Self>, frame: &Rect) {
        if frame == self.absolute_frame() {
            self.cropped = None;
            return;
        }

        let mut this = *self.absolute_frame();
        let mut cropped = *frame;

        cropped.origin -= this.origin;

        this.origin -= this.origin;

        let x_offset = cropped.x() / this.width();
        let y_offset = cropped.y() / this.height();

        let width_shrink = cropped.width() / this.width();
        let height_shrink = cropped.height() / this.height();

        let vertices = image_vertices_with_shrink(x_offset, y_offset, width_shrink, height_shrink);

        let buffer = WGPUApp::device().create_buffer_init(&BufferInitDescriptor {
            label:    "Colored Image Cropped Vertex Buffer".into(),
            contents: cast_slice(&vertices),
            usage:    BufferUsages::VERTEX,
        });

        self.cropped = buffer.into();
    }
}
