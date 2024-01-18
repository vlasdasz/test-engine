use anyhow::Result;
use log::debug;
use rtools::data_manager::DataManager;
use test_engine::{gl_wrapper::system_events::SystemEvents, ui_layer::UILayer, Image, Screen};
use ui::{layout::Anchor, refs::Weak, view, SubView, ViewData, ViewSetup};
use ui_views::{Button, ImageView};

use crate::view_tests::record_touches;

#[view]
struct ImageTestView {
    button:     SubView<Button>,
    image_view: SubView<ImageView>,
}

impl ViewSetup for ImageTestView {
    fn setup(mut self: Weak<Self>) {
        self.button.place.size(100, 100);
        // self.button.set_color(Color::RED).on_tap(|| {
        //     Screen::take_screenshot();
        // });

        SystemEvents::get().after_draw.sub(move || {
            let pos = UILayer::get().cursor_position;
            let color = Screen::read_pixel(pos);
            self.button.set_color(color);
        });

        self.image_view.place.center().relative(Anchor::Size, self, 0.5);
        self.image_view.image = Image::get("square.png");
    }
}

pub async fn test_image_view() -> Result<()> {
    Screen::set_test_view::<ImageTestView>(400, 400).await;

    record_touches().await;

    Screen::take_screenshot();

    debug!("Int view test: OK");

    Ok(())
}
