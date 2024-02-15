use dispatch::on_main;
use gm::{flat::Size, U8Color};
use refs::Weak;
use tokio::spawn;
use ui::{UIEvents, ViewCallbacks, ViewData, ViewSetup};
use ui_proc::view;

use crate as test_engine;
use crate::App;

#[view]
pub struct ColorMeter {
    screenshot:      Vec<U8Color>,
    scrennshot_size: Size<usize>,
}

impl ViewSetup for ColorMeter {
    fn setup(self: Weak<Self>) {
        UIEvents::size_changed().sub(move || self.update_screenshot());
    }
}

impl ViewCallbacks for ColorMeter {
    fn update(&mut self) {
        if self.screenshot.is_empty() {
            return;
        }
        let pos = App::current().cursor_position;
        let pos = Size::<usize>::new(pos.x as _, pos.y as _);
        let index = pos.height * self.scrennshot_size.width + pos.width;
        let Some(color) = self.screenshot.get(index) else {
            return;
        };
        self.set_color(*color);
    }
}

impl ColorMeter {
    fn update_screenshot(mut self: Weak<Self>) {
        spawn(async move {
            let Some((data, size)) = App::read_display().await.ok() else {
                return;
            };

            on_main(move || {
                self.screenshot = data;
                self.scrennshot_size = Size::new(size.width as _, size.height as _);

                // Image::free_with_name("Screenshot");

                // let Some(image) = Image::from_raw_data(
                //     App::state(),
                //     &cast_slice(&self.screenshot),
                //     "Screenshot",
                //     size.into(),
                //     4,
                // )
                // .alert_err() else {
                //     return;
                // };
                // self.image_view.image = image;
            });
        });
    }
}
