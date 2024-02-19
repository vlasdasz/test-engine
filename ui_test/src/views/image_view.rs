use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    sleep,
    ui::{
        view, Anchor, Color, ColorMeter, Container, Image, ImageView, Point, SubView, TouchStack, U8Color,
        ViewCallbacks, ViewData, ViewSetup, ViewTouch,
    },
    App, DataManager,
};

use crate::view_tests::record_touches_with_colors;

#[view]
struct ImageTestView {
    meter:      SubView<ColorMeter>,
    image_view: SubView<ImageView>,
    indicator:  SubView<Container>,
}

impl ViewCallbacks for ImageTestView {
    fn update(&mut self) {
        if self.meter.screenshot_ready() {
            self.indicator.set_color(self.meter.get_pixel(App::current().cursor_position));
        }
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
    let pixel = meter.get_pixel(pos);
    let pixel_f32: Color<f32> = pixel.into();
    let color_f32: Color<f32> = color.into();

    let diff = pixel_f32.diff(color_f32);

    let max_diff = 0.012;

    if diff > max_diff {
        panic!(
            "Color diff is too big: {diff}. Max: {max_diff}. Position: {pos:?}. \nExpected: {color}, got: \
             {pixel}"
        )
    }
}

async fn check_colors(meter: SubView<ColorMeter>, data: &str) {
    if let Some(meter_loaded) = meter.weak().load_receiver() {
        meter_loaded.await.unwrap();
    }

    sleep(0.1);

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
        r#"   59  103 -  25  51  76
             113  104 -  38 207  16
             191  110 -  12  63  40
             269  109 -   1   5 147
             347  132 -  25  51  76
             292  182 -  36  36  67
             277  189 -  25  51  76
             121  190 -  25  51  76
             109  195 -  51  59   5
              81  195 -  25  51  76
              77  281 -  25  51  76
             137  292 - 192  15   4
             251  290 - 209 139  14
             293  256 - 145 145  20
             322  256 -  25  51  76
             259  253 -  25  51  76
             278  318 -  25  51  76
             320  290 -  25  51  76
             186  326 -  25  51  76
             107  316 -  25  51  76
              41  304 -  25  51  76
             154   78 -  25  51  76
             233   83 -  25  51  76
             180  170 -  25  51  76
             220  220 -  25  51  76
        "#,
    )
    .await;

    App::set_window_size((1000, 80));

    record_touches_with_colors(view.meter).await;

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
