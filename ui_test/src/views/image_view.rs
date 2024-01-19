use anyhow::Result;
use log::debug;
use rtools::data_manager::DataManager;
use test_engine::{
    from_main,
    gm::{flat::Point, Color},
    Image, Screen,
};
use ui::{layout::Anchor, refs::Weak, view, SubView, ViewSetup, ViewTouch};
use ui_views::ImageView;

#[view]
struct ImageTestView {
    image_view: SubView<ImageView>,
}

impl ViewSetup for ImageTestView {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();
        self.touch.began.val(|touch| {
            dbg!(touch.position);
            dbg!(Screen::read_pixel(touch.position));
        });

        // SystemEvents::get().after_draw.sub(move || {
        //     let pos = UILayer::get().cursor_position;
        //     let color = Screen::read_pixel(pos);
        //     self.button.set_color(color);
        // });

        self.image_view.place.center().relative(Anchor::Size, self, 0.5);
        self.image_view.image = Image::get("blue.png");
    }
}

async fn check_pixel_color(pos: Point, color: Color) {
    from_main(move || {
        assert_eq!(Screen::read_pixel(pos), color);
    })
    .await
}

async fn check_colors<const N: usize>(data: [((f32, f32), (f32, f32, f32, f32)); N]) {
    for val in data {
        check_pixel_color(
            (val.0 .0, val.0 .1).into(),
            Color::rgba(val.1 .0, val.1 .1, val.1 .2, val.1 .3),
        )
        .await
    }
}

pub async fn test_image_view() -> Result<()> {
    Screen::set_test_view::<ImageTestView>(400, 400).await;

    check_colors([
        ((98.0, 113.0), (0.5019608, 0.5019608, 0.5019608, 1.0)),
        ((105.0, 118.0), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((295.0, 119.0), (0.05882353, 0.1764706, 0.45882353, 1.0)),
        ((304.0, 121.0), (0.5019608, 0.5019608, 0.5019608, 1.0)),
    ])
    .await;

    debug!("Image view test: OK");

    Ok(())
}
