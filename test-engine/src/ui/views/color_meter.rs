use dispatch::on_main;
use refs::Weak;
use tokio::spawn;
use ui::{Setup, UIEvents, ViewCallbacks, ViewData};
use ui_proc::view;
use window::Screenshot;

use crate as test_engine;
use crate::AppRunner;

#[view]
pub struct ColorMeter {
    screenshot: Screenshot,
}

impl Setup for ColorMeter {
    fn setup(self: Weak<Self>) {
        self.update_screenshot();
        UIEvents::size_changed().sub(self, move || self.update_screenshot());
    }
}

impl ViewCallbacks for ColorMeter {
    fn update(&mut self) {
        let pos = AppRunner::cursor_position();

        if pos.is_negative() {
            return;
        }

        self.set_color(self.screenshot.get_pixel(pos));
    }
}

impl ColorMeter {
    pub fn update_screenshot(mut self: Weak<Self>) {
        spawn(async move {
            let Some(screenshot) = AppRunner::take_screenshot().await.ok() else {
                return;
            };

            on_main(move || {
                if self.is_null() {
                    return;
                }

                self.screenshot = screenshot;

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
