use dispatch::on_main;
use gm::{
    flat::{Point, Size},
    num::lossy_convert::LossyConvert,
    Color, U8Color,
};
use log::debug;
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

impl ColorMeter {
    pub fn get_pixel(&self, pos: impl Into<Point>) -> U8Color {
        if self.screenshot.is_empty() {
            return Default::default();
        }

        let pos: Point<usize> = pos.into().lossy_convert();

        let Some(color) = self.screenshot.get(pos.x + pos.y * self.scrennshot_size.width) else {
            return Default::default();
        };

        let color: Color<f32> = (*color).into();

        color.from_srgb().into()
    }
}

impl ViewSetup for ColorMeter {
    fn setup(self: Weak<Self>) {
        self.update_screenshot();
        UIEvents::size_changed().sub(move || self.update_screenshot());
    }
}

impl ViewCallbacks for ColorMeter {
    fn update(&mut self) {
        if self.screenshot.is_empty() {
            return;
        }
        let pos = App::current().cursor_position;

        if pos.negative() {
            return;
        }

        self.set_color(self.get_pixel(pos));
    }
}

impl ColorMeter {
    pub fn update_screenshot(mut self: Weak<Self>) {
        debug!("Update");

        debug!("{:?}", App::root_view_size());

        spawn(async move {
            let Some((data, size)) = App::read_display().await.ok() else {
                return;
            };

            on_main(move || {
                self.screenshot = data;
                self.scrennshot_size = Size::new(size.width as _, size.height as _);

                debug!("Data updated");
                debug!("{:?}", App::root_view_size());

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
