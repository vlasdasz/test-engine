use anyhow::Result;
use log::debug;
use test_engine::{
    from_main,
    refs::Weak,
    ui::{
        view, Anchor, Color, ColorMeter, Container, Image, ImageView, Point, SubView, TouchStack, U8Color,
        ViewCallbacks, ViewData, ViewSetup, ViewTouch,
    },
    wait_for_next_frame, App, DataManager,
};

use crate::view_tests::{record_touches, record_touches_with_colors};

#[view]
struct ImageTestView {
    meter:      SubView<ColorMeter>,
    image_view: SubView<ImageView>,
    indicator:  SubView<Container>,
}

impl ViewCallbacks for ImageTestView {
    fn update(&mut self) {
        self.indicator.set_color(self.meter.get_pixel(App::current().cursor_position));
    }
}

impl ViewSetup for ImageTestView {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();

        self.image_view.place().center().relative(Anchor::Size, self, 0.5);
        self.image_view.image = Image::get("gradient.png");

        self.indicator.place().size(100, 100).br(0);
    }
}

fn check_pixel_color(meter: Weak<ColorMeter>, pos: Point, color: U8Color) {
    let pixel = meter.get_pixel(pos).into();

    let diff = meter.get_pixel(pos).diff(color);

    dbg!(&diff);

    // from_main(move || {
    //     // let diff = Screen::read_pixel(pos).diff(color);
    //     // let max_diff = 0.012;
    //     // if diff > max_diff {
    //     //     panic!("Color diff is too big: {diff}. Max: {max_diff}.
    // Position:     // {pos:?}") }
    // })
    // .await
}

async fn check_colors(meter: SubView<ColorMeter>, data: &str) {
    let lines: Vec<_> = data.split("\n").collect();

    for line in lines {
        let parts: Vec<_> = line.split("-").collect();

        if parts.len() != 2 {
            continue;
        }

        let pos = parts[0];
        let color = parts[1];

        let pos: Vec<_> = pos.split(" ").filter(|a| !a.is_empty()).collect();
        let color: Vec<_> = color.split(" ").filter(|a| !a.is_empty()).collect();

        let pos: Point = Point::new(pos[0].parse().unwrap(), pos[1].parse().unwrap());
        let color: U8Color = U8Color::rgba(
            color[0].parse().unwrap(),
            color[1].parse().unwrap(),
            color[2].parse().unwrap(),
            255,
        );

        check_pixel_color(meter.weak(), pos, color);
    }
}

pub async fn test_image_view() -> Result<()> {
    let view = App::set_test_view::<ImageTestView>(400, 400).await;

    record_touches_with_colors(view.meter).await;

    check_colors(
        view.meter,
        r#"   60  224 -  25  51  76
             108  255 - 120  15   3
             144  230 -  25  51  76
        "#,
    )
    .await;

    record_touches(view).await;

    // from_main(|| {
    //     Screen::current().set_size((200, 600));
    // })
    // .await;

    // from_main(|| {
    //     Screen::current().set_size((600, 250));
    // })
    // .await;

    dbg!(TouchStack::dump());

    debug!("Image view test: OK");

    Ok(())
}
