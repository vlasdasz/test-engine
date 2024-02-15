use std::mem::size_of;

use test_engine::{
    cast_slice, on_main,
    refs::{weak_from_ref, Weak},
    ui::{
        view, AlertErr, Color, Container, Image, ImageView, IntSize, SubView, U8Color, ViewCallbacks,
        ViewData, ViewSetup,
    },
    App,
};
use tokio::spawn;

#[view]
pub struct ColorView {
    green: SubView<Container>,
    blue:  SubView<Container>,

    indicator:  SubView<Container>,
    image_view: SubView<ImageView>,
}

impl ViewSetup for ColorView {
    fn setup(mut self: Weak<Self>) {
        self.green.set_color(Color::GREEN).place().left_half();
        self.blue.set_color(Color::BLUE).place().right_half();
        self.indicator.place().size(40, 40).bl(0);
    }
}

impl ViewCallbacks for ColorView {
    fn update(&mut self) {
        let cursor_pos = App::current().cursor_position;

        let mut this = weak_from_ref(self);
        spawn(async move {
            let Ok((buffer, width_bytes)) = App::request_read_display().await else {
                return;
            };
            let width_colors = width_bytes / size_of::<U8Color>() as u64;

            let bytes: &[u8] = &buffer.slice(..).get_mapped_range();
            let data: &[U8Color] = cast_slice(bytes);

            Image::free()

            let Some(image) = Image::from_raw_data(
                App::state(),
                bytes,
                "Screenshot".to_string(),
                (100, 100).into(),
                4,
            )
            .alert_err() else {
                return;
            };

            // let color = data[(width_colors as f32 * cursor_pos.y) as usize + cursor_pos.x
            // as usize];
            let color = data[cursor_pos.x as usize];
            on_main(move || {
                this.image_view.image = image;
                this.indicator.set_color(color);
            });
        });
    }
}
