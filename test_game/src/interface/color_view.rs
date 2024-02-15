use test_engine::{
    cast_slice, on_main,
    refs::Weak,
    ui::{
        view, AlertErr, Button, Color, Container, Image, ImageView, Size, SubView, U8Color, UIEvents,
        ViewData, ViewSetup,
    },
    App, DataManager,
};
use tokio::spawn;

#[view]
pub struct ColorView {
    green: SubView<Container>,
    blue:  SubView<Container>,

    indicator:  SubView<Container>,
    image_view: SubView<ImageView>,

    update_button: SubView<Button>,
}

impl ColorView {
    fn update_screenshot(mut self: Weak<Self>, size: Size<u32>) {
        spawn(async move {
            let Ok((buffer, _width_bytes)) = App::request_read_display().await else {
                return;
            };

            let bytes: &[u8] = &buffer.slice(..).get_mapped_range();
            let data: Vec<U8Color> = cast_slice(bytes)
                .into_iter()
                .map(|color: &U8Color| color.bgra_to_rgba())
                .collect();

            let bytes = cast_slice(&data).to_vec();

            on_main(move || {
                Image::free_with_name("Screenshot");

                let Some(image) =
                    Image::from_raw_data(App::state(), &bytes, "Screenshot", size.into(), 4).alert_err()
                else {
                    return;
                };
                self.image_view.image = image;
            });
        });
    }
}

impl ViewSetup for ColorView {
    fn setup(mut self: Weak<Self>) {
        self.green.set_color(Color::GRAY_BLUE).place().left_half();
        self.blue.set_color(Color::LIGHT_BLUE).place().right_half();
        self.indicator.place().size(40, 40).bl(0);
        self.image_view.place().size(200, 200).br(0);
        self.update_button.set_text("Update").place().size(200, 50);
        UIEvents::size_changed().val(move |size| self.update_screenshot(size));
    }
}
