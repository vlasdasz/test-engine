use test_engine::{
    cast_slice, on_main,
    refs::Weak,
    ui::{
        view, AlertErr, Button, Color, Container, Image, ImageView, SubView, UIEvents, ViewData, ViewSetup,
    },
    App, DataManager,
};
use tokio::spawn;

#[view]
pub struct ColorView {
    green: SubView<Container>,
    blue:  SubView<Container>,

    image_view: SubView<ImageView>,

    update_button: SubView<Button>,
}

impl ColorView {
    fn update_screenshot(mut self: Weak<Self>) {
        spawn(async move {
            let Some((data, size)) = App::read_display().await.alert_err() else {
                return;
            };

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
        self.image_view.place().size(200, 200).br(0);
        self.update_button.set_text("Update").place().size(200, 50);

        UIEvents::size_changed().sub(move || self.update_screenshot());
    }
}
