use anyhow::Result;
use log::debug;
use test_engine::{
    refs::Weak,
    ui::{
        view, Anchor, Color, Image, ImageView, Point, Screenshot, SubView, TouchStack, U8Color, ViewData,
        ViewSetup, ViewTouch,
    },
    App, DataManager,
};

use crate::view_tests::record_touches_with_colors;

#[view]
struct ImageTestView {
    image_view: SubView<ImageView>,
}

impl ViewSetup for ImageTestView {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();

        self.image_view.place().center().relative(Anchor::Size, self, 0.5);
        self.image_view.image = Image::get("gradient.png");
    }
}

fn check_pixel_color(screenshot: &Screenshot, pos: Point, color: U8Color) {
    let pixel: Color = screenshot.get_pixel(pos).into();
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

async fn check_colors(data: &str) -> Result<()> {
    let screenshot = App::take_screenshot().await?;

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

        check_pixel_color(&screenshot, pos, color);
    }

    Ok(())
}

pub async fn test_image_view() -> Result<()> {
    let view = App::set_test_view::<ImageTestView>(400, 400).await;

    check_colors(
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
    .await?;

    App::set_window_size((1000, 120));

    check_colors(
        r#"  235   42 -  25  51  76
             263   44 -  34 131  10
             301   46 -  25  51  76
             702   52 -  25  51  76
             735   52 -  30  30  77
             781   53 -  25  51  76
             764   83 -  25  51  76
             734   82 - 188 188  19
             674   66 -  25  51  76
             299   60 -  25  51  76
             263   66 -  74  33   4
             379  102 -  25  51  76
             517   90 -  25  51  76
             519   34 -  25  51  76
             524   20 -  25  51  76
        "#,
    )
    .await?;

    App::set_window_size((200, 600));

    check_colors(
        r#"   36  158 -  25  51  76
              62  158 -  33 180  16
              94  158 -  12  66  40
             116  159 -   4  23  88
             129  159 -   2   8 129
             183  165 -  25  51  76
             163  435 -  25  51  76
             121  435 - 207 123  13
              88  435 - 192  38   6
              70  435 - 188  16   4
              29  412 -  25  51  76
              57  248 -  25  51  76
              55  286 -  47  64   6
              82  274 -  25  51  76
             128  333 -  25  51  76
             177  337 -  25  51  76
             127  471 -  25  51  76
             127  405 -  25  51  76
             117  109 -  25  51  76
             106  184 -  25  51  76

        "#,
    )
    .await?;

    debug!("Image view test: OK");

    Ok(())
}
